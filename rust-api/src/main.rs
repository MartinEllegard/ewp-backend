use actix_web::{web, App, HttpServer, Responder};
use std::env;

mod api;
pub mod repository;
pub mod schemas;

//Actix web state
#[derive(Clone)]
pub struct AppState {
    repository: repository::mongodb::Repository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_env_url = env::var("DATABASE_CONNECTION").expect("DATABASE_URL must be set in .env file");
    let repository = repository::mongodb::Repository::new(db_env_url).await;

    let app_state = AppState {
        repository,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
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
