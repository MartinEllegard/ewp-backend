use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnUserSkill {
    pub id: i32,
    pub user_id: i32,
    pub skill_id: i32,
    pub proficiency: i32,
}