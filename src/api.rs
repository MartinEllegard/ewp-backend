use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    auth::{Claims, Jwt},
    schemas, AppState,
};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    //User routes
    cfg.service(web::resource("/register").route(web::post().to(register_user)))
        .service(
            web::resource("/profiles")
                .route(web::get().to(get_profiles))
                .route(web::post().to(post_profile))
                .route(web::put().to(put_profile_by_id)),
        )
        .service(
            web::resource("/profiles/id/{id}").route(web::get().to(get_profile_by_id)), //.route(web::put().to(profiles::put_user))
        )
        .service(
            web::resource("/profiles/me")
                .route(web::get().to(get_profile_by_user))
                .route(web::put().to(put_profile_self)),
        )
        .service(
            web::resource("/profiles/skills/{skills_string}")
                .route(web::get().to(get_profiles_by_skills)),
        )
        .service(web::resource("/auth").route(web::post().to(authenticate_user)));
}

pub async fn register_user(
    data: web::Data<AppState>,
    new_user_no_id_json: web::Json<schemas::UserNoId>,
) -> impl Responder {
    let new_user_no_id = new_user_no_id_json.into_inner();
    let new_user = schemas::User {
        id: Uuid::new_v4().to_string(),
        username: new_user_no_id.username,
        password: new_user_no_id.password,
    };
    let result = data.repository.register_user(new_user).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Error registering user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn authenticate_user(
    data: web::Data<AppState>,
    user: web::Json<schemas::UserNoId>,
) -> impl Responder {
    let result = data
        .repository
        .authenticate_user(&user.username, &user.password)
        .await;
    match result {
        Ok(Some(user)) => {
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let expiration = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize)
                + 3600; // 1 hour
            let claims = Claims::new(user.id, expiration);
            match claims.encode(secret.as_bytes()) {
                Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
                Err(e) => {
                    eprintln!("Error encoding token: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            eprintln!("Error authenticating user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_profiles(app_state: web::Data<AppState>, _: Jwt) -> HttpResponse {
    let profiles = app_state.repository.get_all_profiles().await;
    match profiles {
        Ok(profiles) => HttpResponse::Ok().json(profiles),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn post_profile(
    app_state: web::Data<AppState>,
    profile_json: web::Json<schemas::ProfileNoId>,
    jwt: Jwt,
) -> HttpResponse {
    let id = Uuid::parse_str(&jwt.0.as_str()).unwrap_or(Uuid::new_v4());
    let profile_no_id = profile_json.into_inner();
    let profile = schemas::Profile {
        id,
        name: profile_no_id.name,
        email: profile_no_id.email,
        description: profile_no_id.description,
        company: profile_no_id.company,
        age: profile_no_id.age,
        projects: profile_no_id.projects,
        certificates: profile_no_id.certificates,
        experience: profile_no_id.experience,
        education: profile_no_id.education,
        skills: profile_no_id.skills,
    };
    let result = app_state.repository.create_profile(profile).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn put_profile_self(
    app_state: web::Data<AppState>,
    profile_json: web::Json<schemas::Profile>,
    jwt: Jwt,
) -> HttpResponse {
    let id = Uuid::parse_str(&jwt.0.as_str());
    let profile = profile_json.into_inner();
    match id {
        Ok(id) => {
            if id == profile.id {
                let result = app_state.repository.update_profile(id, profile).await;

                match result {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                }
            } else {
                HttpResponse::BadRequest().body("Id does not match logged in user")
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn put_profile_by_id(
    app_state: web::Data<AppState>,
    profile_json: web::Json<schemas::Profile>,
    _: Jwt,
) -> HttpResponse {
    let profile = profile_json.into_inner();
    let id = profile.id.clone();

    if id == profile.id {
        let result = app_state.repository.update_profile(id, profile).await;

        match result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Id not match posted user")
    }
}

pub async fn get_profile_by_id(
    app_state: web::Data<AppState>,
    profile_id: web::Path<Uuid>,
    _: Jwt,
) -> HttpResponse {
    let profile = app_state
        .repository
        .get_profiles_by_id(profile_id.into_inner())
        .await;
    match profile {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_profile_by_user(app_state: web::Data<AppState>, jwt: Jwt) -> HttpResponse {
    let id = Uuid::parse_str(&jwt.0.as_str());
    match id {
        Ok(id) => {
            let profile = app_state.repository.get_profiles_by_id(id).await;
            match profile {
                Ok(profile) => HttpResponse::Ok().json(profile),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn get_profiles_by_skills(
    app_state: web::Data<AppState>,
    skills_string: web::Path<String>,
    _: Jwt,
) -> HttpResponse {
    let profile = app_state
        .repository
        .get_profiles_by_skills(skills_string.into_inner())
        .await;
    match profile {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
