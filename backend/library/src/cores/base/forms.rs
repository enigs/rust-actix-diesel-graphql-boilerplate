use async_graphql::{MaybeUndefined, InputObject};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::Base, error = "BaseError")]
#[serde(rename_all = "camelCase")]
pub struct BaseForm {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub api_url: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub web_url: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub admin_url: MaybeUndefined<String>,
}