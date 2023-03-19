use std::sync::Arc;

use crate::model::user::{User, UserCompany, UserProject, UserSkill};
use actix_web::Error;
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};

const USER_COLLECTION_NAME: &'static str = "users";

#[derive(Serialize, Deserialize, Debug)]
struct UserWrapper {
    user: User
}

pub struct Firedb {
    db: FirestoreDb,
}

impl Firedb {
    pub async fn new(project_id: &str) -> Self {
        let firebase = FirestoreDb::new(project_id).await;
        match firebase {
            Ok(db) => Firedb { db },
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub async fn add_user(&self, user: &User) -> Result<(), Error> {
        let wrapped_user = UserWrapper { user: user.clone() };
        let object_returned = self.db.fluent()
                    .insert()
                    .into(USER_COLLECTION_NAME)
                    .document_id(&user.uuid)
                    .object(&wrapped_user.user)
                    .execute()
                    .await;
        return true;
    }

    pub async fn update_user(&self, user: &User) -> Result<(), Error> {
        let wrapped_user = UserWrapper { user: user.clone() };
        let path = format!("users/{}", user.uuid);

        self.db.patch(&path, &wrapped_user).await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_uuid: &str) -> Result<(), Error> {
        let path = format!("users/{}", user_uuid);

        self.db.delete(&path).await?;
        Ok(())
    }
}