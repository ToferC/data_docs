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
    author_id: Uuid,
    subject: Uuid,
    category: Uuid,
    searchable_text_en: String,
    searchable_text_fr: String,
    keyword_ids: Vec<Uuid>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "metadata"]
pub struct InsertableMetaData {
    document_id: Uuid,
    author_id: Uuid,
    subject: Uuid,
    category: Uuid,
    searchable_text_en: String,
    searchable_text_fr: String,
    keyword_ids: Vec<Uuid>
}

impl MetaData {
    pub fn create(
        md: &InsertableMetaData,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(metadata::table)
            .values(md)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_by_id(id: Uuid) -> Result<MetaData, CustomError> {
        let conn = database::connection()?;

        let document = metadata::table
            .filter(metadata::id.eq(id))
            .first::<Self>(&conn)?;

        Ok(document)
    }

    pub fn get_by_document_id(document_id: Uuid) -> Result<MetaData, CustomError> {
        let conn = database::connection()?;

        let document = metadata::table
            .filter(metadata::document_id.eq(document_id))
            .first::<Self>(&conn)?;

        Ok(document)
    }

    pub fn update(metadata: &MetaData) -> Result<Self, CustomError> {
        let conn = metadata::connection()?;
        let v = diesel::update(metadata::table)
            .filter(metadata::id.eq(metadata.id))
            .set(metadata)
            .get_result(&conn)?;
        Ok(v)
    }
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