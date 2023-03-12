use super::skill::BaseSkill;

pub struct BaseUser {
    pub user_uuid: String,
    pub description: String,
    pub name: String,
    pub email: String,
    pub age: i32,
}

pub struct UserSkill {
    pub skill_uuid: String,
    pub name: String,
    pub description: String,
    pub level: i32,
}

pub struct UserCompany {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

pub struct UserProject {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<BaseSkill>>,
}

pub struct User {
    pub user_uuid: String,
    pub description: String,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub skills: Option<Vec<UserSkill>>,
    pub projects: Option<Vec<UserProject>>,
    pub companies: Option<Vec<UserCompany>>,
}