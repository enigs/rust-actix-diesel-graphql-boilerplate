use serde_json::Value;

use crate::Mailer;
use crate::MailerAttachment;

/// Setters for mailer
impl Mailer {
    pub fn set_attachments(&mut self, attachments: Vec<MailerAttachment>) -> &mut Self {
        self.attachments = Some(attachments);

        self
    }

    /// Set context
    pub fn set_context(&mut self, context: Value) -> &mut Self {
        self.context = context;

        self
    }

    /// Set template
    pub fn set_template<T>(&mut self, template: T) -> &mut Self
        where T: ToString
    {
        self.template = template.to_string();

        self
    }
}