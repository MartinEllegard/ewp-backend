use actix_web::{web, HttpResponse};
//use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;

use crate::{schemas, AppState};

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
