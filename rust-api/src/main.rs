use std::env;
use actix_web::{web, App, HttpServer, Responder};
use sqlx::postgres::{PgPoolOptions, PgPool};

pub mod schemas;
mod routes;

//Actix web state
#[derive(Clone)]
pub struct AppState {
    pool: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_env_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_env_url.as_str())
        .await
        .unwrap();

    let app_state = AppState {
        pool: pool,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(index))
            .route("/users", web::get().to(routes::user::get_users))
            .route("/users/{id}", web::get().to(routes::user::get_user))
            .route("/users", web::post().to(routes::user::post_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world!"
}