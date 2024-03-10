use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum NoteError {
    NotFound,
    CouldNotCreate,
}

impl ResponseError for NoteError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NoteError::NotFound => StatusCode::NOT_FOUND,
            NoteError::CouldNotCreate => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
