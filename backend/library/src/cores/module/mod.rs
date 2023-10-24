use macros::SetEnum;

#[derive(Default, Debug, Clone, Copy, PartialEq, SetEnum)]
#[derive(diesel::expression::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum Module {
    #[default]
    Base,
    Mailer,
    Paseto,
    S3
}