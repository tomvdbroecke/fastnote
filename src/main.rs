// Modules
mod db;
mod error;
mod models;

// Uses
use crate::db::Database;
use crate::models::{CreateNoteRequest, Note, UpdateNotesUrl};
use actix_web::web::Data;
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    HttpServer,
};
use error::NoteError;
use uuid::Uuid;
use validator::Validate;

// @todo:
// - Add validation error handling

// GET /notes (move this later)
#[get("/notes")]
async fn get_notes(db: Data<Database>) -> Result<Json<Vec<Note>>, NoteError> {
    let notes = db.get_notes().await;
    match notes {
        Some(notes) => Ok(Json(notes)),
        None => Err(NoteError::NotFound),
    }
}

// POST /notes (move this later)
#[post("/notes")]
async fn create_note(
    body: Json<CreateNoteRequest>,
    db: Data<Database>,
) -> Result<Json<Note>, NoteError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let note_title = body.title.clone();
            let note_body = body.body.clone();

            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_note = db
                .create_note(Note::new(new_uuid.to_string(), note_title, note_body))
                .await;

            match new_note {
                Some(note) => Ok(Json(note)),
                None => Err(NoteError::CouldNotCreate),
            }
        }
        Err(_) => Err(NoteError::CouldNotCreate),
    }
}

// PATCH /notes (move this later)
#[patch("/notes/{uuid}")]
async fn update_note(
    update_notes_url: Path<UpdateNotesUrl>,
    db: Data<Database>,
) -> Result<Json<Note>, NoteError> {
    let uuid = update_notes_url.into_inner().uuid;
    let update_result = db.update_note(uuid).await;

    match update_result {
        Some(note) => Ok(Json(note)),
        None => Err(NoteError::NotFound),
    }
}

// @todo:
// - Can we get rid of the clone?
// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    let db = Database::init()
        .await
        .expect("Failed to initialize the database");
    let db_data = Data::new(db);

    // Start the server
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(db_data.clone())
            .service(get_notes)
            .service(create_note)
            .service(update_note)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
