use diesel::{AsExpression, FromSqlRow};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[serde(rename_all = "camelCase")]
pub struct MailerAttachment {
    pub filename: String,
    pub name: String
}