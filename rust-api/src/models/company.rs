use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCompany {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnCompany {
    pub id: i32,
    pub name: String,
}

pub struct CompanyWithUsers {
    pub id: i32,
    pub name: String,
    pub users: Vec<super::user::ReturnUser>,
}