use serde::{Serialize, Deserialize};

use super::skill::BaseSkill;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseUser {
    pub user_uuid: String,
    pub description: String,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSkill {
    pub skill_uuid: String,
    pub name: String,
    pub description: String,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCompany {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProject {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<BaseSkill>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub uuid: String,
    pub description: String,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub skills: Option<Vec<UserSkill>>,
    pub projects: Option<Vec<UserProject>>,
    pub companies: Option<Vec<UserCompany>>,
}