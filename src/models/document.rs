use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use chrono::prelude::*;


use crate::database;
use crate::schema::{documents, sections};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "documents"]
pub struct Document {
    pub id: Uuid,
    pub title_text_id: Uuid,
    pub purpose_text_id: Uuid,
    // pub approvals: Option<Vec<Uuid>>, // Replace with Approvals
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub publishable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// A human readable Template
pub struct ReadableDocument {
    pub id: Uuid,
    pub title_text: String,
    pub purpose_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "documents"]
pub struct InsertableDocument {
    pub title_text_id: Uuid,
    pub purpose_text_id: Uuid,
}

impl InsertableDocument {
    pub fn new(
        raw_title_text: String,
        raw_purpose_text: String,
        lang: String,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            lang.to_owned(), 
            raw_title_text.to_owned(),
            None);

        let title_text = Text::create(&insertable_name_text)?;

        let insertable_purpose_text = InsertableText::new(
            lang.to_owned(), 
            raw_purpose_text,
            None);

        let purpose_text = Text::create(&insertable_purpose_text)?;

        Ok(InsertableDocument {
            title_text_id: title_text.id,
            purpose_text_id: purpose_text.id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "sections"]
pub struct Section {
    pub id: Uuid,
    pub document_id: Uuid,
    pub template_section_id: Uuid, // References the template section so we don't duplicate the data
}

