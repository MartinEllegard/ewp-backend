use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnSkill {
    pub name: String,
    pub skill_user: String,
}
