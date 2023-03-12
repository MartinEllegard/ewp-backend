use super::{user::BaseUser, skill::BaseSkill};

pub struct BaseProject {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub skills: Vec<BaseSkill>,
    pub users: Option<Vec<BaseUser>>,
}