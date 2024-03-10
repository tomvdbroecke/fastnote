// Modules
mod db;
mod models;

// Uses
use crate::db::Database;
use crate::models::{CreateNoteRequest, Note, UpdateNotesUrl};
use actix_web::web::Data;
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    HttpResponse, HttpServer, Responder,
};
use uuid::Uuid;
use validator::Validate;

// GET /notes (move this later)
#[get("/notes")]
async fn get_notes(db: Data<Database>) -> impl Responder {
    let notes = db.get_notes().await;
    match notes {
        Some(notes) => HttpResponse::Ok().json(notes),
        None => HttpResponse::InternalServerError().finish(),
    }
}

// POST /notes (move this later)
#[post("/notes")]
async fn create_note(body: Json<CreateNoteRequest>, db: Data<Database>) -> impl Responder {
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
                Some(note) => HttpResponse::Ok().json(note),
                None => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

// PATCH /notes (move this later)
#[patch("/notes/{uuid}")]
async fn update_note(update_notes_url: Path<UpdateNotesUrl>) -> impl Responder {
    let uuid = update_notes_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating the note with uuid: {uuid}"))
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
