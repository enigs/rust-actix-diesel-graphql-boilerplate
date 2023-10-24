use async_graphql::{MaybeUndefined, InputObject};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::S3, error = "S3Error")]
#[serde(rename_all = "camelCase")]
pub struct S3Form {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_key_id: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub secret_access_key: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub bucket: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub path: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub region: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_small_size: MaybeUndefined<i32>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_small_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_small_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xxxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xxxl_size: MaybeUndefined<i32>,
}