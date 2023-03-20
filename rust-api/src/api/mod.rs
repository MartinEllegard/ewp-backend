use actix_web::web;


pub mod profiles;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    //User routes
    cfg.service(
        web::resource("/profiles")
            .route(web::get().to(profiles::get_profiles))
            .route(web::post().to(profiles::post_profile))
    );
    cfg.service(
        web::resource("/profiles/{id}")
            .route(web::get().to(profiles::get_profile_by_id))
            //.route(web::put().to(profiles::put_user))
    );
    // cfg.service(
    //     web::resource("/profiles/{id}/skills")
    //         .route(web::get().to(profiles::get_user_skills))
    // );
}