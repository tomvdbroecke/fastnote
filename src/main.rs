// Modules
mod db;
mod models;

// Uses
use crate::db::Database;
use crate::models::{CreateNoteRequest, UpdateNotesUrl};
use actix_web::web::Data;
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    HttpResponse, HttpServer, Responder,
};
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
async fn create_notes(body: Json<CreateNoteRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().body("Note validated"),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

// PATCH /notes (move this later)
#[patch("/notes/{uuid}")]
async fn update_notes(update_notes_url: Path<UpdateNotesUrl>) -> impl Responder {
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
            .service(create_notes)
            .service(update_notes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
