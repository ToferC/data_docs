use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub id: Uuid,
    // Content_id for the majority of user-entered texts. Exceptions are for texts about documents
    // Might want to make this a different data type
    pub content_id: Option<Uuid>,
    pub lang: String,
    pub content: String,
    pub translated: bool,
    pub machine_translation: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextForm {
    pub lang: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertableText {
    pub lang: String,
    pub content: String,
    pub machine_translation: bool,
}

impl InsertableText {
    pub fn new(lang: &str, content: String, machine_translation: bool) -> Self {
        InsertableText {
            lang: lang.to_string(),
            content,
            machine_translation,
        }
    }
}