use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnUserSkill {
    pub id: i32,
    pub user_id: i32,
    pub skill_id: i32,
    pub proficiency: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillForUser {
    pub id: i32,
    pub name: String,
    pub proficiency: i32,
}
