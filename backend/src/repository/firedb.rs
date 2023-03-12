use firestore::*;

pub struct FireStore {
    db: FirestoreDb
}

impl FireStore {
    pub async fn init(project_id: String) -> FireStore {
        let db = FirestoreDb::new(project_id).await.unwrap();

        FireStore {
            db
        }
    }
}
