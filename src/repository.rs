use mongodb::{options::ClientOptions, Client, Collection, bson::{doc, Document, from_document, to_document}};
use uuid::Uuid;
use futures::stream::StreamExt;
use pwhash::bcrypt;

use crate::schemas;

const DB_NAME: &str = "ewp-db";

#[derive(Clone)]
pub struct Repository {
    client: Client,
}

impl Repository {
    pub async fn new(connection_string: String) -> Self {
        let client_options = ClientOptions::parse(&connection_string).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        Repository { client }
    }

    fn init_db(&self, collection_name: &str) -> Collection<Document> {
        let db = self.client.database(DB_NAME);
        db.collection(collection_name)
    }

    pub async fn register_user(&self, user: schemas::User) -> Result<(), mongodb::error::Error> {
        let coll = self.init_db("users");

        let filter = doc! {"username": &user.username};
        let doc = coll.find_one(filter, None).await?;
        if doc.is_some() {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::Io(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "User already exists").into())));
        }
        

        let hashed_password = bcrypt::hash(&user.password).expect("Failed to hash pasword");
        let user = schemas::User {
            password: hashed_password,
            ..user
        };
        let doc = to_document(&user).expect("Failed to convert user to document");
        coll.insert_one(doc, None).await?;

        Ok(())
    }

    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<schemas::User>, mongodb::error::Error> {
        let coll = self.init_db("users");
        let filter = doc! {"username": username};
        let doc = coll.find_one(filter, None).await?;

        if let Some(doc) = doc {
            let user: schemas::User = from_document(doc)?;
            if bcrypt::verify(password, &user.password) {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    pub async fn find_user_by_id(&self, user_id: &str) -> Result<Option<schemas::User>, mongodb::error::Error> {
        let coll = self.init_db("users");
        let filter = doc! {"id": user_id};
        let doc = coll.find_one(filter, None).await?;

        if let Some(doc) = doc {
            let user: schemas::User = from_document(doc)?;
            return Ok(Some(user));
        }
        Ok(None)
    }

    pub async fn create_profile(&self, profile: schemas::Profile) -> Result<(), mongodb::error::Error> {
        let exists = self.check_profile_exist(&profile).await?;
        if exists {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::Io(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Profile already exists").into())));
        }
        let doc = to_document(&profile).expect("Failed to convert profile to document");
        let coll = self.init_db("profiles");
        coll.insert_one(doc, None).await?;

        Ok(())
    }

    pub async fn get_all_profiles(&self) -> Result<Vec<schemas::Profile>, mongodb::error::Error> {
        let coll = self.init_db("profiles");
        let mut cursor = coll.find(None, None).await?;
        let mut results = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let profile: schemas::Profile = from_document(doc)?;
                    results.push(profile);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(results)
    }

    pub async fn get_profiles_by_id(&self, profile_id: Uuid) -> Result<schemas::Profile, mongodb::error::Error> {
        let coll = self.init_db("profiles");
        let filter = doc! {"id": profile_id.to_string()};
        let result = coll.find_one(filter, None).await?;
        match result {
            Some(doc) => {
                let profile: schemas::Profile = from_document(doc)?;
                Ok(profile)
            }
            None => {
                Err(mongodb::error::Error::from(mongodb::error::ErrorKind::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Not found").into())))
            }
        }
    }

   pub async fn get_profiles_by_skills(&self, skills_string: String) -> Result<Vec<schemas::Profile>, mongodb::error::Error> {
        let skills: Vec<String> = skills_string.split(',').map(|s| s.trim().to_string()).collect();
        let coll = self.init_db("profiles");
        let filter = doc! {"skills": {"$elemMatch": { "name": { "$in": skills } } } };
        let mut cursor = coll.find(filter, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let profile: schemas::Profile = from_document(doc)?;
                    results.push(profile);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(results)
    }

    pub async fn update_profile(&self, profile_id: Uuid, update_doc: Document) -> Result<(), mongodb::error::Error> {
        let coll = self.init_db("profiles");
        let filter = doc! {"id": profile_id.to_string()};
        coll.update_one(filter, update_doc, None).await?;
        Ok(())
    }

    pub async fn delete_profile(&self, profile_id: Uuid) -> Result<(), mongodb::error::Error> {
        let coll = self.init_db("profiles");
        let filter = doc! {"id": profile_id.to_string()};
        coll.delete_one(filter, None).await?;
        Ok(())
    }

    pub async fn check_profile_exist(&self, profile: &schemas::Profile) -> Result<bool, mongodb::error::Error> {
        let coll = self.init_db("profiles");
        let filter = doc! {"name": &profile.name};

        let doc = coll.find_one(filter, None).await?;
        Ok(doc.is_some())
    }
}
