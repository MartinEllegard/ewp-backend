use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use strum::{Display};

#[derive(Deserialize, Serialize)]
pub struct UserIdentifier {
    user_global_id: String,
}

#[get("/user/{user_global_id}")]
pub async fn get_user(user_identifier: Path<UserIdentifier>) -> Json<String> {
    Json(user_identifier.into_inner().user_global_id)
}
