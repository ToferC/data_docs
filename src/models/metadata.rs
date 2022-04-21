use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{QueryDsl};
use chrono::prelude::*;
use std::collections::BTreeMap;
use pulldown_cmark::{html, Options, Parser};

use crate::{database, process_text_redactions};
use crate::schema::{documents, template_sections, texts, sections};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text, TemplateSection,
    ReadableTemplateSection, User, Section, ReadableSection};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "metadata"]
pub struct MetaData {
    id: Uuid,
    document_id: Uuid,
    subject: Uuid,
    category: Uuid,
    keyword_ids: Vec<Uuid>
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "subjects"]
pub struct Subject {
    id: Uuid,
    en_string: String,
    fr_string: String,
    en_description: Option<String>,
    fr_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "categories"]
pub struct Category {
    id: Uuid,
    en_string: String,
    fr_string: String,
    en_description: Option<String>,
    fr_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "keywords"]
pub struct KeyWord {
    id: Uuid,
    en_string: String,
    fr_string: String,
    en_description: Option<String>,
    fr_description: Option<String>,
}