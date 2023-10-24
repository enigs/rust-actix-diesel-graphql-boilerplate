use actix_files::{ Files, NamedFile };
use actix_web::{HttpRequest, HttpResponse, Result, get, guard, web::{self, Data}, Responder};
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::Compress;
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{ GraphQLRequest, GraphQLResponse };
use std::path::PathBuf;
use std::sync::Arc;

use library::parsers;
use library::{Core, sse::Broadcaster};
use resolver::ProjectSchema;

// Get: / - Create index page as health check
#[get("/")]
pub async fn health_check(core: Data<Arc<Core>>) -> String {
    let locales = Arc::clone(&core.locale);

    locales.lookup_with_args("version", &[
        ("version", env!("CARGO_PKG_VERSION"))
    ])
}

// Create favicon fileserver handler
#[get("/favicon.{ext}/")]
pub async fn favicon() -> Result<NamedFile> {
    let path: PathBuf = "./assets/static/media/favicon.ico".to_string().parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// Create static fileserver handler
pub fn static_files() -> impl HttpServiceFactory {
    web::scope("/static")
        .wrap(Compress::default())
        .service(Files::new("/", "./assets/static/").index_file("error/404.html"))
}

// Set playground
pub fn playground() -> impl HttpServiceFactory {
    web::resource("/public/")
        .guard(guard::Get())
        .to(playground_page)
}

// Expose playground
pub async fn playground_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/public").finish())
}

// Set resolver
pub fn resolvers() -> impl HttpServiceFactory {
    web::resource("/public/")
        .guard(guard::Post())
        .to(resolvers_page)
}

// Create resolver handler for post queries
pub async fn resolvers_page(core:Data<Arc<Core>>, schema: Data<ProjectSchema>, req: HttpRequest, gql: GraphQLRequest) -> GraphQLResponse {
    let uap = core.user_agent_parser();

    schema.execute(parsers::graphql(&req, gql, uap))
        .await
        .into()
}

#[get("/events/")]
async fn events(broadcaster: Data<Arc<Broadcaster>>) -> impl Responder {
    broadcaster.new_client("test").await
}

#[get("/broadcast/{ty}/{msg}/")]
async fn broadcast(
    broadcaster: Data<Arc<Broadcaster>>,
    actix_web_lab::extract::Path((ty, msg,)): actix_web_lab::extract::Path<(String, String,)>,
) -> impl Responder {
    broadcaster.broadcast("test", &ty, &msg).await;
    HttpResponse::Ok().body("msg sent")
}

