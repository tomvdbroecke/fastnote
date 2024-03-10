use actix_web::{get, patch, post, HttpResponse, HttpServer, Responder};

#[get("/notes")]
async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("GET /notes")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| actix_web::App::new().service(get_notes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
