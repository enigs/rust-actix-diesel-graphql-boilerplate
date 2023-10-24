pub mod base;
pub mod database;
pub mod mailer;
pub mod module;
pub mod locale;
pub mod paseto;
pub mod s3;
pub mod schema;

use async_graphql::{Context, Result};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use user_agent_parser::UserAgentParser;

use crate::Base;
use crate::DBManager;
use crate::Errors;
use crate::Locale;
use crate::Mailer;
use crate::Paseto;
use crate::Response;
use crate::S3;

/// Core struct - contains core libraries
/// Locales - internationalization for the entire graphql system
/// Base - base settings
/// Mailer - mailer settings & functionalities
/// Paseto - paseto settings & functionalities
/// S3 - s3 settings & functionalities
pub struct Core {
    pub base: Arc<RwLock<Base>>,
    pub database: DBManager,
    pub locale: Arc<Locale>,
    pub mailer: Arc<RwLock<Mailer>>,
    pub paseto: Arc<RwLock<Paseto>>,
    pub s3: Arc<RwLock<S3>>,
    pub user_agent_parser: UserAgentParser
}

impl Core {
    pub fn init(embedded_migration: EmbeddedMigrations) -> anyhow::Result<Arc<Self>> {
        let locale = Arc::new(Locale::default());

        let user_agent_parser = UserAgentParser::from_path(config::USER_AGENT_REGEXES)
            .unwrap_or_else(|_| { panic!("{}", locale.lookup("user-agent-parser-init-failed")) });

        let database = DBManager::init()
            .unwrap_or_else(|_| { panic!("{}", locale.lookup("database-init-failed")) });

        // Run pending migrations
        if database.writer().run_pending_migrations(embedded_migration).is_err() {
            panic!("{}", locale.lookup("database-migration-failed"));
        }

        let base = Base::init(&database)?;
        let mailer = Mailer::init(&database)?;
        let paseto = Paseto::init(&database)?;
        let s3 = S3::init(&database)?;

        Ok(Arc::new(Self {
            database,
            locale,
            user_agent_parser,
            base,
            mailer,
            paseto,
            s3
        }))
    }

    pub fn base<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, Base>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("base-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.base.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn database<'a>(ctx: &Context<'a>) -> Result<&'a DBManager> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("db-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            return Ok(&settings.database);
        }

        Err(Errors::to(response, error))
    }

    pub fn locales<'a>(ctx: &Context<'a>) -> Result<&'a Arc<Locale>> {
        let response = Response::InternalServerError;
        let error = "Unable to retrieve locale settings";

        match ctx.data_opt::<Arc<Self>>() {
            Some(settings) => Ok(&settings.locale),
            None => Err(Errors::to(response, error))
        }
    }

    pub fn mailer<'a>(ctx: &'a Context<'a>) -> Result<Mailer> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("mailer-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.mailer.try_read() {
                return Ok(settings.clone());
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn paseto<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, Paseto>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("base-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.paseto.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn s3<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, S3>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("s3-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.s3.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn user_agent_parser(&self) -> &UserAgentParser {
        &self.user_agent_parser
    }
}