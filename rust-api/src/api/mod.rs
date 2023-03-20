use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;


pub mod profiles;
pub mod auth;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    //User routes
    cfg
    .service(
        web::resource("/register")
            .route(web::post().to(auth::register_user)),
    )
    .service(
        web::resource("/profiles")
            .route(web::get().to(profiles::get_profiles))
            .route(web::post().to(profiles::post_profile))
    )
    .service(
        web::resource("/profiles/{id}")
            .route(web::get().to(profiles::get_profile_by_id))
            //.route(web::put().to(profiles::put_user))
    )
    .service(
        web::resource("/auth")
            .route(web::post().to(auth::authenticate_user)),
    );
    // cfg.service(
    //     web::resource("/profiles/{id}/skills")
    //         .route(web::get().to(profiles::get_user_skills))
    // );
}