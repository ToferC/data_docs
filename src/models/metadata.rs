use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{QueryDsl};
use chrono::prelude::*;
use std::collections::BTreeMap;
use pulldown_cmark::{html, Options, Parser};
use pithy::{Summariser};
use std::sync::Arc;

use crate::{database, process_text_redactions};
use crate::schema::{metadata};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text, TemplateSection, Document,
    ReadableTemplateSection, User, Section, ReadableSection,
    machine_translate_string};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "metadata"]
pub struct MetaData {
    id: Uuid,
    searchable_title_en: String,
    searchable_title_fr: String,
    document_id: Uuid,
    author_id: Uuid,
    subject_id: Uuid,
    category_id: Uuid,
    summary_text_en: String,
    summary_text_fr: String,
    keyword_ids: Vec<Uuid>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "metadata"]
pub struct InsertableMetaData {
    document_id: Uuid,
    searchable_title_en: String,
    searchable_title_fr: String,
    author_id: Uuid,
    subject_id: Uuid,
    category_id: Uuid,
    summary_text_en: String,
    summary_text_fr: String,
    keyword_ids: Vec<Uuid>
}

impl InsertableMetaData {
    pub async fn new(document_id: Uuid, lang: &str) -> Result<Self, CustomError> {

        let (document, sections) = Document::get_all_readable_by_id(
            document_id, 
            lang,
            false,
            false,
        )
            .expect("Unable to retrieve readable document");

        let mut text = String::new();

        for (id, s) in sections {
            text.push_str(&s.content);
            text.push_str("\n");
        };

        let mut summariser = Summariser::new();

        summariser.add_raw_text(
            document.title_text.clone(),
            text,
            ". ",
            30,
            1500,
            false,
        );

        let top_sentences = summariser.approximate_top_sentences(
            1,
            3.,
            1.5,
        );

        let mut summary = String::new();

        for ts in top_sentences {
            summary.push_str(&ts.text);
        };

        let mut translation_text = Vec::new();

        translation_text.push(document.title_text.clone());
        translation_text.push(summary.clone());

        let l = Arc::new(lang.to_owned().clone());

        let translations = machine_translate_string(
            Arc::new(translation_text),
            l,
        ).await?;
           
        let (en_title, fr_title) = match lang {
            "en" => (document.title_text.clone(), translations[0].clone()),
            _ => (translations[0].clone(), document.title_text.clone())
        };

        let (en_summary, fr_summary) = match lang {
            "en" => (summary.clone(), translations[1].clone()),
            _ => (translations[1].clone(), summary.clone())
        };


        let meta = InsertableMetaData {
            document_id,
            searchable_title_en: en_title,
            searchable_title_fr: fr_title,
            author_id: document.created_by_id,
            subject_id: Uuid::new_v4(),
            category_id: Uuid::new_v4(),
            summary_text_en: en_summary,
            summary_text_fr: fr_summary,
            keyword_ids: vec![Uuid::new_v4()],
        };

        Ok(meta)
    }
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
        let conn = database::connection()?;
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