use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use crate::{schemas, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    company_id: Option<i32>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

pub async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users = sqlx::query_as!(
        schemas::User,
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

pub async fn get_user(app_state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let user = sqlx::query_as!(
        schemas::User,
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

pub async fn post_user(app_state: web::Data<AppState>, user_json: web::Json<User>) -> HttpResponse {
    let user = user_json.into_inner();

    let user_exist = sqlx::query_as!(
        schemas::User,
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

    let new_user = sqlx::query_as!(
        schemas::User,
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

    match new_user {
        Ok(new_user) => {
            HttpResponse::Ok().json(new_user)
        },
        Err(e) => {
            HttpResponse::Ok().body(e.to_string())
        }
    }
}