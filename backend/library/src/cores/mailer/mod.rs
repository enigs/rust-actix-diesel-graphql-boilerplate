pub mod attachment;
pub mod fields;
pub mod forms;
pub mod impls;
pub mod init;
pub mod queries;
pub mod setters;
pub mod validations;

use arraygen::Arraygen;
use diesel::{AsExpression, FromSqlRow};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use macros::{
    AsJsonb,
    SetCipher,
    SetIsEmpty,
    SetMutate
};

use crate::ciphers;
use crate::MailerAttachment;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsExpression, FromSqlRow)]
#[derive(AsJsonb, SetCipher, SetIsEmpty, SetMutate)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
pub struct Mailer {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub smtp_host: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub service: String,
    #[serde(default, skip_serializing)]
    pub template: String,
    #[serde(default, skip_serializing)]
    pub context: Value,
    #[serde(default, skip_serializing)]
    pub attachments: Option<Vec<MailerAttachment>>
}
