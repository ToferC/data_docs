use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub purpose_text_id: Uuid,
    pub approvals: Vec<Uuid>, // Replace with Approvals
    pub publishable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub document_id: Uuid,
    pub template_section_id: Uuid, // References the template section so we don't duplicate the data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub id: Uuid,
    pub section_id: Uuid,
}

