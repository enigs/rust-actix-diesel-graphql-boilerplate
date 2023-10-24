pub mod fields;
pub mod forms;
pub mod impls;
pub mod init;
pub mod queries;
pub mod validations;

use arraygen::Arraygen;
use diesel::{AsExpression, FromSqlRow};
use serde::{Serialize, Deserialize};

use macros::{
    AsJsonb,
    SetCipher,
    SetIsEmpty,
    SetMutate
};

use crate::ciphers;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen, AsExpression, FromSqlRow)]
#[derive(AsJsonb, SetCipher, SetIsEmpty, SetMutate)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub api_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub web_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub admin_url: String
}


