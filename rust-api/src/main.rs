use std::env;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgPoolOptions, PgPool};

//Actix web state
#[derive(Clone)]
struct AppState {
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
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(post_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    company_id: Option<i32>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users")
        .fetch_all(&app_state.pool)
        .await;

    match users {
        Ok(users) => {
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            HttpResponse::Ok().body(e.to_string())
        }
    }
}

async fn get_user(app_state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        id)
        .fetch_one(&app_state.pool)
        .await;

    match user {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            HttpResponse::Ok().body(e.to_string())
        }
    }
}

async fn post_user(app_state: web::Data<AppState>, user: web::Json<User>) -> HttpResponse {
    let user_exist = sqlx::query!(
        "SELECT * FROM users WHERE email = $1",
        user.email)
        .fetch_one(&app_state.pool)
        .await;

    match user_exist {
        Ok(_) => {
            return HttpResponse::Conflict().body("User already exist");
        },
        Err(_) => {}
    }

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (firstname, lastname, description, email, company_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        user.firstname,
        user.lastname,
        user.description,
        user.email,
        user.company_id,
        user.created_at,
        user.updated_at)
        .fetch_one(&app_state.pool)
        .await;

    match user {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            HttpResponse::Ok().body(e.to_string())
        }
    }
}