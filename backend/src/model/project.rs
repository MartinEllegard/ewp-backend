use serde::{Serialize, Deserialize};
use super::{user::BaseUser, skill::BaseSkill};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseProject {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub skills: Vec<BaseSkill>,
    pub users: Option<Vec<BaseUser>>,
}