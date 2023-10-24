/// App related variables
pub const APP_NAME: &str = "Qsweep";

/// Base path variable for routes
pub const BASE_PATH: &str = "";

/// Set address
pub const SERVER_ADDRESS: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 9020;

/// Set assets variables
pub const FROM_FAVICON: &str = "/favicon.ico";
pub const FROM_STATIC: &str = "/static";

pub const PATH_FAVICON: &str = "assets/static/media/favicon.ico";
pub const PATH_STATIC: &str = "assets/static";
pub const PATH_ERROR_404: &str = "assets/templates/errors/404.html";

/// Set attempt variables
pub const ATTEMPT_RETRY_MAX: usize = 5;
pub const ATTEMPT_RETRY_DURATION: usize = 30;

/// Set catchers variables
pub const CATCHER_CACHE_DIRECTIVES: u32 = 86400u32;
pub const CATCHER_MIME_HTML: &str = "text/html; charset=utf-8";
pub const CATCHER_TEMPLATE_404_PATH: &str = "errors/404.html";

/// CORS related variables
pub const CORS_METHODS: [&str; 5] = ["GET", "POST", "PATCH", "DELETE", "OPTIONS"];

/// Cron related variables
// pub const CRON_DURATION: &str = "0 0 0-23 * * * *"; // Every Hour
pub const CRON_DURATION: &str = "0 0 */1 * * * *"; // Every Hour
// pub const CRON_DURATION: &str = "0 0-59 * * * * *"; // every minute
// pub const CRON_DURATION: &str = "0 */1 * * * * *"; // every minute
// pub const CRON_DURATION: &str = "0 */2 * * * * *"; // every 2 minutes

/// Set handlebars variables
pub const HANDLEBARS_ASSET_PATH: &str = "./assets/templates";
pub const HANDLEBARS_EXTENSION: &str = ".hbs";

/// Locales related variables
pub const LOCALES_PATH: &str = "./assets/locales/";
pub const LOCALES_US: &str = "en-US";

/// Mailer related variables
pub const MAILER_FROM_HELLO: &str = "Qsweep <hello@qsweep.com>";
pub const MAILER_FROM_NO_REPLY: &str = "Qsweep <no-reply@qsweep.com>";
pub const MAILER_FROM_SUCCESS: &str = "Qsweep <success@qsweep.com>";
pub const MAILER_TO_CONTROLLER: &str = "markhenry.liwag@gmail.com";

/// Paseto defaults
pub const PASETO_ACCESS_TOKEN_KEY_UNIT: &str = "120";
pub const PASETO_ACCESS_TOKEN_KEY_TIME: &str = "Days";
pub const PASETO_ACCESS_TOKEN_KEY_SIGNING: &str = "BX8hllVNjp5IbB2NiUlt7OUctq71PKSqkMD9p1NovcY";
pub const PASETO_REFRESH_TOKEN_KEY_UNIT: &str = "180";
pub const PASETO_REFRESH_TOKEN_KEY_TIME: &str = "Days";
pub const PASETO_REFRESH_TOKEN_KEY_SIGNING: &str = "-Xs6DCM7vQ9yKJX2uCQBgpqnWSyqDCGZu0Uej6sUjxQ";

/// Sentry related variables
pub const SENTRY_URL: &str = "https://a9b6160b38114b9b88141671e025db90@o1152810.ingest.sentry.io/4505390121680896";

/// User Agent Parser related variables
pub const USER_AGENT_REGEXES: &str = "./assets/regexes.yaml";

/// Retrieve CORS
pub fn cors() -> actix_cors::Cors {
    // Set bindings
    let binding = CORS_METHODS.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let methods: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

    // Return cors
    actix_cors::Cors::default()
        .allow_any_origin()
        .allowed_methods(methods)
        .allow_any_header()
        .max_age(3600)
}

/// Return environment filter value
pub fn env_filter() -> tracing_subscriber::EnvFilter {
    let level = match std::env::var("TRACING_LEVEL") {
        Ok(level) => level,
        Err(_) => "error".to_string()
    };

    tracing_subscriber::EnvFilter::from(level)
}

/// Retrieve handlebars
pub fn handlebars() -> hbs::Handlebars<'static> {
    let mut handlebars = hbs::Handlebars::new();

    handlebars
        .register_templates_directory(HANDLEBARS_EXTENSION, HANDLEBARS_ASSET_PATH)
        .expect("Invalid template directory");

    handlebars
}

pub async fn not_found_page() -> actix_web::Result<actix_web::HttpResponse> {
    let body = handlebars()
        .render(CATCHER_TEMPLATE_404_PATH, &None::<String>)
        .expect("Invalid template directory");

    Ok(actix_web::HttpResponse::build(actix_web::http::StatusCode::NOT_FOUND)
        .insert_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::Public,
            actix_web::http::header::CacheDirective::MaxAge(CATCHER_CACHE_DIRECTIVES),
        ]))
        .content_type(CATCHER_MIME_HTML)
        .body(body))
}
