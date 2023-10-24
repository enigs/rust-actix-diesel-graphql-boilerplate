use async_graphql::Result;
use infer::Infer;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use serde_json::Value;

use crate::Errors;
use crate::Mailer;
use crate::Response;

impl Mailer {
    pub fn to<T: From<Self>>(&self) -> T {
        T::from(self.clone())
    }

    pub fn filter(&mut self) -> &mut Self {
        self.context = Value::Null;
        self.template = String::default();
        self.attachments = None;

        self
    }

    pub fn send<F, T, S>(&mut self, from: F, to: T, subject: S) -> Result<String>
        where F: ToString,
              T: ToString,
              S: ToString
    {
        // Make sure values were properly decrypted
        let data = self.decrypt();

        // Set bindings
        let from = from.to_string();
        let to = to.to_string();
        let subject = subject.to_string();

        // Set html email body
        let body = config::handlebars()
            .render(&data.template, &data.context)
            .map_err(|e| Errors::to(Response::InternalServerError, e.to_string()))?;

        let username = data.username.clone();
        let password = data.password.clone();
        let smtp_host = data.smtp_host.clone();

        // Create multipart body
        let mut multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body)
            );

        // Check if attachment is available
        if let Some(attachments) = data.attachments.clone() {
            for attachment in attachments {
                let filename = attachment.filename.clone();
                let name = attachment.name.clone();

                match std::fs::read(&filename) {
                    Ok(file) => {
                        let info = Infer::new();
                        match ContentType::parse(&info
                            .get(&file.clone())
                            .map_or(String::default(), |t| String::from(t.mime_type()))) {
                            Ok(content_type) => {
                                multipart = multipart.singlepart(
                                    Attachment::new(name).body(file, content_type)
                                );
                            },
                            Err(_) => continue
                        };
                    },
                    Err(_) => continue
                }
            }
        }


        // Create email builder
        let builder = match Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart) {
            Ok(builder) => builder,
            Err(error) => return Err(Errors::to(Response::BadRequest, error.to_string()))
        };

        // Set credentials
        let credentials = Credentials::new(username, password);

        // Set smtp transport relay
        let relay = match SmtpTransport::relay(smtp_host.as_str()) {
            Ok(relay) => relay,
            Err(error) => return Err(Errors::to(Response::BadRequest, error.to_string()))
        };

        // Open a remote connection
        let mailer = relay.credentials(credentials).build();

        // Send the email
        match mailer.send(&builder) {
            Ok(_) => Ok(format!("Email sent successfully to {to}")),
            Err(error) => Err(Errors::to(Response::BadRequest, error.to_string())),
        }
    }
}