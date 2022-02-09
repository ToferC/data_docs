use uuid::Uuid;
use chrono::NaiveDateTime;

pub struct Decision {
    id: Uuid,
    document_id: Uuid,
    decision: String,
    date_time: NaiveDateTime,
}