use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUser {
    pub firstname: String,
    pub lastname: String,
    pub description: String,
    pub email: String,
    pub company_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnUser {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub description: String,
    pub email: String,
    pub company_id: Option<i32>,
}