use uuid::Uuid;
use chrono::NaiveDateTime;

pub struct Text {
    pub id: Uuid,
    pub lang: String,
    pub text: String,
    pub machine_translation: bool,
    pub approved: bool,
    pub created_at: NaiveDateTime,
}