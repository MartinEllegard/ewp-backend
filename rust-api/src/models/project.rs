use serde::{Serialize, Deserialize};
use super::{user::ReturnUser, skill::ReturnSkill};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnProject {
    pub name: String,
    pub description: String,
    pub users: Option<ReturnUser>,
    pub skills: Option<ReturnSkill>
}
