
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub description: String,
    pub email: String,
    pub company_id: Option<i32>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    id: i32,
    name: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthAccessToken {
    id: i32,
    user_id: i32,
    accesstoken: String,
    expires_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    id: i32,
    name: String,
    description: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    id: i32,
    name: String,
    skill_use: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillUser {
    id: i32,
    user_id: i32,
    skill_id: i32,
    proficiency: i32,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSkill {
    id: i32,
    project_id: i32,
    skill_id: i32,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectUser {
    id: i32,
    user_id: i32,
    project_id: i32,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillCorrelation {
    id: i32,
    skill_id: i32,
    skill_id2: i32,
    correlation: f32,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}