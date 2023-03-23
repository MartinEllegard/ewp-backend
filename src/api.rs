use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{web, Responder, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, schemas, auth::Claims};
//use actix_web_httpauth::extractors::bearer::BearerAuth;


pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    //User routes
    cfg
    .service(
        web::resource("/register")
            .route(web::post().to(register_user)),
    )
    .service(
        web::resource("/profiles")
            .route(web::get().to(get_profiles))
            .route(web::post().to(post_profile))
    )
    .service(
        web::resource("/profiles/{id}")
            .route(web::get().to(get_profile_by_id))
            //.route(web::put().to(profiles::put_user))
    )
    .service(
        web::resource("/profiles/skills/{skills_string}")
            .route(web::get().to(get_profiles_by_skills)),
    )
    .service(
        web::resource("/auth")
            .route(web::post().to(authenticate_user)),
    );
    // cfg.service(
    //     web::resource("/profiles/{id}/skills")
    //         .route(web::get().to(profiles::get_user_skills))
    // );
}

pub async fn register_user(data: web::Data<AppState>, new_user_no_id_json: web::Json::<schemas::UserNoId>) -> impl Responder {
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

pub async fn authenticate_user(data: web::Data<AppState>, user: web::Json::<schemas::UserNoId>) -> impl Responder {
    let result = data.repository.authenticate_user(&user.username, &user.password).await;
    match result {
        Ok(Some(user)) => {
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let expiration = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize) + 3600; // 1 hour
            let claims = Claims::new(user.id, expiration);
            match claims.encode(secret.as_bytes()) {
                Ok(token) => {
                    HttpResponse::Ok().json(json!({ "token": token }))
                },
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

// Update the update_profile handler to check for user authorization
// pub async fn update_profile(
//     data: web::Data<AppState>,
//     jwt: Jwt, // Add Jwt extractor here
//     path: web::Path<Uuid>,
//     update_doc: web::Json<Document>,
// ) -> impl Responder {
//     let profile_id = path.into_inner();
//     let profile = data.repository.get_profile_by_id(profile_id).await;
//     match profile {
//         Ok(Some(profile)) => {
//             if jwt.0 == profile.user_id {
//                 match data.repository.update_profile(profile_id, update_doc.into_inner()).await {
//                     Ok(_) => HttpResponse::Ok().finish(),
//                     Err(e) => {
//                         eprintln!("Error updating profile: {}", e);
//                         HttpResponse::InternalServerError().finish()
//                     }
//                 }
//             } else {
//                 HttpResponse::Forbidden().finish()
//             }
//         }
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(e) => {
//             eprintln!("Error getting profile: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

pub async fn get_profiles(app_state: web::Data<AppState>) -> HttpResponse {
    let profiles = app_state.repository.get_all_profiles().await;
    match profiles {
        Ok(profiles) => {
            HttpResponse::Ok().json(profiles)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn post_profile(app_state: web::Data<AppState>, profile_json: web::Json<schemas::ProfileNoId>) -> HttpResponse {
    let profile_no_id = profile_json.into_inner();
    let profile = schemas::Profile{
        id: Uuid::new_v4(),
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
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn get_profile_by_id(app_state: web::Data<AppState>, profile_id: web::Path<Uuid>) -> HttpResponse {
    let profile = app_state.repository.get_profiles_by_id(profile_id.into_inner()).await;
    match profile {
        Ok(profile) => {
            HttpResponse::Ok().json(profile)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn get_profiles_by_skills(app_state: web::Data<AppState>, skills_string: web::Path<String>) -> HttpResponse {
    let profile = app_state.repository.get_profiles_by_skills(skills_string.into_inner()).await;
    match profile {
        Ok(profile) => {
            HttpResponse::Ok().json(profile)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
