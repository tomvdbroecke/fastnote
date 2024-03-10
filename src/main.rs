use actix_web::{get, patch, post, HttpResponse, HttpServer, Responder};

#[get("/notes")]
async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("GET /notes")
}

#[post("/notes")]
async fn create_notes() -> impl Responder {
    HttpResponse::Ok().body("POST /notes")
}

#[patch("/notes")]
async fn update_notes() -> impl Responder {
    HttpResponse::Ok().body("PATCH /notes")
}

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
