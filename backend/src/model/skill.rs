use serde::{Serialize, Deserialize};
use super::project::BaseProject;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseSkill {
    pub skill_uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillUser {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub email: String,
    pub age: i32,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub skill_uuid: String,
    pub name: String,
    pub description: String,
    pub users: Option<Vec<SkillUser>>,
    pub projects: Option<Vec<BaseProject>>,
}