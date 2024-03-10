use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::note::Note;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

// @todo:
// - Move signin credentials to .env
// - Move server address to .env
// - Handle the unwrap
impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("notes").await.unwrap();
        Ok(Database {
            client,
            name_space: "surreal".to_string(),
            db_name: "notes".to_string(),
        })
    }

    pub async fn get_notes(&self) -> Option<Vec<Note>> {
        let result = self.client.select("notes").await;
        match result {
            Ok(notes) => Some(notes),
            Err(_) => None,
        }
    }
}
