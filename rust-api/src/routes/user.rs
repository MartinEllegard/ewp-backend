use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

use crate::{schemas, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct InputUser {
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    company_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    firstname: String,
    lastname: String,
    description: String,
    email: String,
    company_id: Option<i32>,
}

pub async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users = sqlx::query_as!(
        schemas::User,
        "SELECT * FROM users")
        .fetch_all(&app_state.pool)
        .await;

    match users {
        Ok(users) => {
            let out = users.into_iter().map(|user| {
                User {
                    id: user.id,
                    firstname: user.firstname,
                    lastname: user.lastname,
                    description: user.description,
                    email: user.email,
                    company_id: user.company_id,
                }
            }).collect::<Vec<User>>();
            HttpResponse::Ok().json(out)
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
            HttpResponse::Ok().json(User {
                id: user.id,
                firstname: user.firstname,
                lastname: user.lastname,
                description: user.description,
                email: user.email,
                company_id: user.company_id,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn post_user(app_state: web::Data<AppState>, user_json: web::Json<InputUser>) -> HttpResponse {
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
        time::OffsetDateTime::now_utc(),
        time::OffsetDateTime::now_utc())
        .fetch_one(&app_state.pool)
        .await;

    match new_user {
        Ok(new_user) => {
            HttpResponse::Ok().json(User {
                id: new_user.id,
                firstname: new_user.firstname,
                lastname: new_user.lastname,
                description: new_user.description,
                email: new_user.email,
                company_id: new_user.company_id,
            })
        },
        Err(e) => {
            HttpResponse::Ok().body(e.to_string())
        }
    }
}