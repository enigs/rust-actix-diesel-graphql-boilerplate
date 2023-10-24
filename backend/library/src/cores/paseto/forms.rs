use async_graphql::{MaybeUndefined, InputObject};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::Paseto, error = "PasetoError")]
#[serde(rename_all = "camelCase")]
pub struct PasetoForm {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub app_name: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui642str)]
    #[error(String)]
    pub access_token_key_unit: MaybeUndefined<i64>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_token_key_time: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_token_key_signing: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui642str)]
    #[error(String)]
    pub refresh_token_key_unit: MaybeUndefined<i64>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub refresh_token_key_time: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub refresh_token_key_signing: MaybeUndefined<String>,
}