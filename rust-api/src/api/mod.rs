use actix_web::web;


pub mod user;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    //User routes
    cfg.service(
        web::resource("/users")
            .route(web::get().to(user::get_users))
            .route(web::post().to(user::post_user))
    );
    cfg.service(
        web::resource("/users/{id}")
            .route(web::get().to(user::get_user))
            .route(web::put().to(user::put_user))
    );
    cfg.service(
        web::resource("/users/{id}/skills")
            .route(web::get().to(user::get_user_skills))
    );
}