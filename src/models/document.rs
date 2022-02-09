use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memo {
    pub id: Uuid,
    pub lang: String,
    pub purpose: String,
    pub title_id: Uuid,
    pub issue_id: Uuid,
    pub background_id: Uuid,
    pub recommendation_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct MemoForm {
    pub lang: String,
    pub purpose: String,
    pub title: String,
    pub issue: String,
    pub background: String,
    pub recommendation: String,
    pub updated_at: String,
    pub author_id: String,
}

