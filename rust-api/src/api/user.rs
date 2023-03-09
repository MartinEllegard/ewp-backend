use actix_web::{web, HttpResponse};

use crate::{schemas, AppState, models::{self, project::ReturnProject}};

pub async fn get_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users = sqlx::query_as!(
        schemas::User,
        "SELECT * FROM users")
        .fetch_all(&app_state.pool)
        .await;
    match users {
        Ok(users) => {
            let model = users.into_iter().map(|user| {
                models::user::ReturnUser {
                    id: user.id,
                    firstname: user.firstname,
                    lastname: user.lastname,
                    description: user.description,
                    email: user.email,
                    company_id: user.company_id,
                }
            }).collect::<Vec<models::user::ReturnUser>>();

            HttpResponse::Ok().json(model)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
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
            HttpResponse::Ok().json(models::user::ReturnUser {
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

pub async fn post_user(app_state: web::Data<AppState>, user_json: web::Json<models::user::PostUser>) -> HttpResponse {
    let user = user_json.into_inner();

    let user_exist = sqlx::query_as!(
        schemas::User,
        "SELECT * FROM users WHERE email = $1",
        user.email)
        .fetch_one(&app_state.pool)
        .await;

    if user_exist.is_ok(){
        return HttpResponse::Conflict().body("User already exist");
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
            HttpResponse::Ok().json(models::user::ReturnUser {
                id: new_user.id,
                firstname: new_user.firstname,
                lastname: new_user.lastname,
                description: new_user.description,
                email: new_user.email,
                company_id: new_user.company_id,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn put_user (app_state: web::Data<AppState>, path: web::Path<i32>, user_json: web::Json<models::user::PostUser>) -> HttpResponse {
    let id = path.into_inner();
    let user = user_json.into_inner();

    let user_exist = sqlx::query_as!(
        schemas::User,
        "SELECT * FROM users WHERE id = $1",
        id)
        .fetch_one(&app_state.pool)
        .await;

    if user_exist.is_err(){
        return HttpResponse::NotFound().body("User not found");
    }

    let updated_user = sqlx::query_as!(
        schemas::User,
        "UPDATE users SET firstname = $1, lastname = $2, description = $3, email = $4, company_id = $5, updated_at = $6 WHERE id = $7 RETURNING *",
        user.firstname,
        user.lastname,
        user.description,
        user.email,
        user.company_id,
        time::OffsetDateTime::now_utc(),
        id)
        .fetch_one(&app_state.pool)
        .await;

    match updated_user {
        Ok(updated_user) => {
            HttpResponse::Ok().json(models::user::ReturnUser {
                id: updated_user.id,
                firstname: updated_user.firstname,
                lastname: updated_user.lastname,
                description: updated_user.description,
                email: updated_user.email,
                company_id: updated_user.company_id,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

// Get user skills
pub async fn get_user_skills (app_state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let user_exist = sqlx::query_as!(
        schemas::User,
        "SELECT * FROM users WHERE id = $1",
        id)
        .fetch_one(&app_state.pool)
        .await;

    if user_exist.is_err() {
        return HttpResponse::NotFound().body("User not found");
    }

    let user_skills = sqlx::query_as!(
        schemas::SkillUser,
        "SELECT * FROM skills_users WHERE user_id = $1",
        id)
        .fetch_all(&app_state.pool)
        .await;

    match user_skills {
        Ok(user_skills) => {
            let model = user_skills.into_iter().map(|user_skill| {
                models::user_skill::ReturnUserSkill {
                    id: user_skill.id,
                    user_id: user_skill.user_id,
                    skill_id: user_skill.skill_id,
                    proficiency: user_skill.proficiency,
                }
            }).collect::<Vec<models::user_skill::ReturnUserSkill>>();

            HttpResponse::Ok().json(model)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
