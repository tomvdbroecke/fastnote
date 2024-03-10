// Modules
mod models;

// Uses
use actix_web::{get, patch, post, web::Json, HttpResponse, HttpServer, Responder};
use models::CreateNoteRequest;
use validator::Validate;

// GET /notes (move this later)
#[get("/notes")]
async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("GET /notes")
}

// POST /notes (move this later)
#[post("/notes")]
async fn create_notes(body: Json<CreateNoteRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().body("Note validated"),
        Err(e) => HttpResponse::BadRequest().body(format!("Validation failed: {}", e)),
    }
}

// PATCH /notes (move this later)
#[patch("/notes/{uuid}")]
async fn update_notes() -> impl Responder {
    HttpResponse::Ok().body("PATCH /notes")
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(get_notes)
            .service(create_notes)
            .service(update_notes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
