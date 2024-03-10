use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Display, Serialize)]
pub enum NoteError {
    #[display(fmt = "Could not find note")]
    NotFound,
    #[display(fmt = "Could not create note")]
    CouldNotCreate,
}

impl ResponseError for NoteError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = json!({ "error": self.to_string() });

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NoteError::NotFound => StatusCode::NOT_FOUND,
            NoteError::CouldNotCreate => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
