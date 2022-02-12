use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
/// Core data structure which to build a Document
///Intent here is that users can create template with all the elements to enable other users
/// to build a document. An example is a memo for approval. The memo has
/// * Title
/// * Issue
/// * Background
/// * Recommendation
/// And an approval section.
/// 
/// When a user is creating a memo, they will see a screen with a single markdown window for each section. They will enter 
/// the text in the markdown window and when saved, the text will be saved to the database and the structural data will be 
/// entered to recreate and manage the document.
pub struct Template {
    pub id: Uuid,
    pub name_text_id: Uuid,
    pub purpose_text_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateSection {
    pub id: Uuid,
    pub document_id: Uuid,
    pub header_text_id: Uuid,
    pub order: usize,
    pub instructions_text_id: Uuid,
    pub help_text_text_id: Uuid,
    pub character_limit: usize,
}
