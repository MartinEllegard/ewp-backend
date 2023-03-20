use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{web, Responder, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, schemas, auth::Claims};

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