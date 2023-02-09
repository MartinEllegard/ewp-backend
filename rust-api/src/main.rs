use core::time;

use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgPoolOptions, PgPool};
use chrono::{NaiveDateTime, Utc};

//Actix web state
#[derive(Clone)]
struct AppState {
    pool: PgPool
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:PassW0rd@localhost:5432/postgres")
        .await
        .unwrap();

    let app_state = AppState {
        pool: pool,
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    company_id: i32,
    created_at: time,
    updated_at: time
}

#[get("/users")]
async fn users(app_state: web::Data<AppState>) -> impl Responder {
    let users = sqlx::query!(
        Vec<User>,
        "SELECT * FROM users")
        .fetch_all(&app_state.pool)
        .await
        .unwrap();

    format!("{:?}", users)
}