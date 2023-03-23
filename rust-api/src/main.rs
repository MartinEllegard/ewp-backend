use actix_web::{web, App, HttpServer, Responder};
use log;
use std::env;

mod api;
pub mod auth;
pub mod repository;
pub mod schemas;

//Actix web state
#[derive(Clone)]
pub struct AppState {
    repository: repository::Repository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();
    let db_env_url =
        env::var("DATABASE_CONNECTION").expect("DATABASE_URL must be set in .env file");
    let repository = repository::Repository::new(db_env_url).await;

    let app_state = AppState { repository };

    log::info!("Starting server at 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").configure(api::scoped_config))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world!"
}
