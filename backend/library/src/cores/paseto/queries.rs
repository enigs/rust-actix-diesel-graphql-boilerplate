use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use nanoid::nanoid;

use crate::cores::schema;
use crate::DBManager;
use crate::Paseto;
use crate::Module;

impl Paseto {
    pub fn select(manager: &DBManager) -> Result<Self> {
        // Retrieve reader
        let mut reader = manager.reader();

        if let Ok(mut settings) = schema::settings::table
            .filter(schema::settings::module.eq(Module::Paseto))
            .select(schema::settings::content)
            .first::<Paseto>(&mut reader) {
            return Ok(settings.decrypt().clone());
        }

        // Return error to generate a new paseto settings
        Err(anyhow::anyhow!("paseto settings not found"))
    }

    pub fn upsert(&mut self, manager: &DBManager) -> Result<Self> {
        // Retrieve writer
        let mut writer = manager.writer();

        // Set content
        let content = self.encrypt();

        // Create settings struct
        #[derive(Debug, Clone, PartialEq, diesel::Insertable)]
        #[diesel(table_name = schema::settings)]
        pub struct Settings<'a> {
            pub id: String,
            pub created_at: DateTime<Utc>,
            pub updated_at: DateTime<Utc>,
            pub module: Module,
            pub content: &'a Paseto,
        }

        // Create settings
        let settings = Settings {
            id: nanoid!(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            module: Module::Paseto,
            content
        };

        // Upsert settings
        diesel::insert_into(schema::settings::table)
            .values(settings.clone())
            .on_conflict(schema::settings::module)
            .do_update()
            .set(schema::settings::content.eq(settings.content))
            .execute(&mut writer)
            .map_err(anyhow::Error::new)?;

        // Return content
        Ok(self.decrypt().clone())
    }
}