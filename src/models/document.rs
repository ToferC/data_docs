use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;

use crate::database;
use crate::schema::{documents, sections};
use crate::errors::CustomError;

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "documents"]
pub struct Document {
    pub id: Uuid,
    pub purpose_text_id: Uuid,
    // pub approvals: Option<Vec<Uuid>>, // Replace with Approvals
    pub publishable: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "sections"]
pub struct Section {
    pub id: Uuid,
    pub document_id: Uuid,
    pub template_section_id: Uuid, // References the template section so we don't duplicate the data
}

