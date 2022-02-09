use chrono::NaiveDateTime;
use rocket::form::{FromForm};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memo {
    pub id: Uuid,
    pub lang: String,
    pub text_id: Uuid,
    pub machine_translation: bool,
    pub approved: bool,
    pub created_at: NaiveDateTime,
}
#[derive(Debug, FromForm)]
pub struct MemoForm {
    pub title: String,
    pub issue: String,
    pub background: String,
    pub updated_at: String,
    pub author_id: String,
}

