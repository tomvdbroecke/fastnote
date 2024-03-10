use crate::models::note::Note;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

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
        let result = self.client.select("note").await;
        match result {
            Ok(notes) => Some(notes),
            Err(_) => None,
        }
    }

    pub async fn create_note(&self, new_note: Note) -> Option<Note> {
        let created_note = self
            .client
            .create(("note", &new_note.uuid))
            .content(new_note)
            .await;
        match created_note {
            Ok(note) => note,
            Err(_) => None,
        }
    }

    pub async fn update_note(&self, uuid: String) -> Option<Note> {
        let find_note: Result<Option<Note>, Error> = self.client.select(("note", &uuid)).await;
        match find_note {
            Ok(found) => match found {
                Some(_) => {
                    let updated_note: Result<Option<Note>, Error> = self
                        .client
                        .update(("note", &uuid))
                        .merge(Note {
                            uuid,
                            title: "Updated Title".to_string(),
                            body: "Updated Body".to_string(),
                        })
                        .await;
                    match updated_note {
                        Ok(note) => note,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
