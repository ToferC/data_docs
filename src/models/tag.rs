use chrono::NaiveDateTime;

pub struct Tag {
    id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}