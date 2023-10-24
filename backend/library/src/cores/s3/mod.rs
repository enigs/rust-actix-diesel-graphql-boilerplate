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
pub struct S3 {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub access_key_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub secret_access_key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub bucket: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub region: String,
    pub image_thumbnail_small_size: i32,
    pub image_thumbnail_medium_size: i32,
    pub image_thumbnail_large_size: i32,
    pub image_thumbnail_xl_size: i32,
    pub image_landscape_width_small_size: i32,
    pub image_landscape_height_small_size: i32,
    pub image_landscape_width_medium_size: i32,
    pub image_landscape_height_medium_size: i32,
    pub image_landscape_width_large_size: i32,
    pub image_landscape_height_large_size: i32,
    pub image_landscape_width_xl_size: i32,
    pub image_landscape_height_xl_size: i32,
    pub image_landscape_width_xxl_size: i32,
    pub image_landscape_height_xxl_size: i32,
    pub image_landscape_width_xxxl_size: i32,
    pub image_landscape_height_xxxl_size: i32,
}