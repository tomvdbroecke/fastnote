use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 1000))]
    pub body: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateNotesUrl {
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Note {
    pub uuid: String,
    pub title: String,
    pub body: String,
}

impl Note {
    pub fn new(uuid: String, title: String, body: String) -> Self {
        Note { uuid, title, body }
    }
}
