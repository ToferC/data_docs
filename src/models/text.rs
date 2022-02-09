use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub id: Uuid,
    pub lang: String,
    pub text: String,
    pub machine_translation: bool,
    pub approved: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertableText {
    pub lang: String,
    pub text: String,
    pub machine_translation: bool,
}

impl InsertableText {
    pub fn new(lang: &str, text: String, machine_translation: bool) -> Self {
        InsertableText {
            lang: lang.to_string(),
            text,
            machine_translation,
        }
    }
}