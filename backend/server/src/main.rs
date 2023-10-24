pub mod pages;

use actix::Actor;
use actix_web::{App, HttpServer, web::{self, Data}};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{JsonStorageLayer, BunyanFormattingLayer};
use tracing_log::LogTracer;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

use library::Core;
use library::{sse::Broadcaster, scheduler::Scheduler};

// Set migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../assets/migrations/");

// Do not use async or framework's macro here as per sentry's instruction
// https://docs.sentry.io/platforms/rust/
// Lazy initialize sentry
static SENTRY: Lazy<sentry::ClientInitGuard> = Lazy::new(|| {
    sentry::init((config::SENTRY_URL, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }))
});

// Lazy initialize broadcaster
static BROADCASTER: Lazy<Arc<Broadcaster>> = Lazy::new(|| {
    Broadcaster::create()
});

// Lazy initialize core modules
static CORE: Lazy<Arc<Core>> = Lazy::new(|| {
    let core = Core::init(MIGRATIONS)
        .expect("Core library initialization failed...");

    Scheduler::builder()
        .set_core(&core)
        .set_duration(config::CRON_DURATION)
        .clone()
        .start();

    core
});

// Async Multi-Threaded Main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenv().ok();

    // Access lazy initialization of sentry
    let _ = &*SENTRY;

    // Initialize log tracer
    if let Err(logger) = LogTracer::init() {
        sentry::capture_error(&logger);
    }

    // Set tracing subscriber
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(
        config::APP_NAME.to_string(),
        non_blocking_writer
    );

    // Initialize tracing
    let subscriber = Registry::default()
        .with(config::env_filter())
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);

    // Set global default subscriber
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Set lazily initialized core & broadcaster
    let core = &*CORE;
    let broadcaster = &*BROADCASTER;

    // Retrieve graphql schemas
    let schema = resolver::schema(core, broadcaster);

    // Instantiate actix web server
    HttpServer::new(move || {
        App::new()
            // Include app data
            .app_data(Data::new( Arc::clone(core)))
            .app_data(Data::new(Arc::clone(broadcaster)))
            .app_data(Data::new(schema.clone()))

            // Set cors
            .wrap(config::cors())

            // Wrap middlewares
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(NormalizePath::new(TrailingSlash::Always))

            // Include routes
            .service(
                web::scope(config::BASE_PATH)
                    .service(pages::health_check)
                    .service(pages::favicon)
                    .service(pages::events)
                    .service(pages::broadcast)
                    .service(pages::static_files())
                    .service(pages::playground())
                    .service(pages::resolvers())
            )

            // Set default service
            .default_service(web::route().to(config::not_found_page))
    })
    .bind((config::SERVER_ADDRESS, config::SERVER_PORT))?
    .run()
    .await
}

