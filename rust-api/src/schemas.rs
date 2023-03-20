
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: u32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub position: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub age: u32,
    pub email: String,
    pub company: String,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
    pub certificates: Vec<Certificate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileNoId {
    pub name: String,
    pub description: String,
    pub age: u32,
    pub email: String,
    pub company: String,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
    pub certificates: Vec<Certificate>,
}