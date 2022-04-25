use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{QueryDsl};
use chrono::prelude::*;
use pithy::{Summariser};
use std::sync::Arc;

use crate::{database};
use crate::schema::{metadata, categories, subjects, keywords};
use crate::errors::CustomError;
use crate::models::{Document, machine_translate_string};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "metadata"]
pub struct MetaData {
    pub id: Uuid,
    pub searchable_title_en: String,
    pub searchable_title_fr: String,
    pub document_id: Uuid,
    pub author_id: Uuid,
    pub subject_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub summary_text_en: String,
    pub summary_text_fr: String,
    pub keyword_ids: Option<Vec<Uuid>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[table_name = "metadata"]
pub struct InsertableMetaData {
    pub document_id: Uuid,
    pub searchable_title_en: String,
    pub searchable_title_fr: String,
    pub author_id: Uuid,
    pub subject_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub summary_text_en: String,
    pub summary_text_fr: String,
    pub keyword_ids: Option<Vec<Uuid>>
}

impl InsertableMetaData {
    pub async fn update_document(document_id: Arc<Uuid>, lang: Arc<String>) -> Result<Self, CustomError> {

        let l = &*lang.as_str();

        let (document, sections) = Document::get_all_readable_by_id(
            *document_id, 
            l,
            false,
            false,
        )
            .expect("Unable to retrieve readable document");

        let mut text = String::new();

        for (_id, s) in sections {
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

        let translations = machine_translate_string(
            Arc::new(translation_text),
            Arc::new(l.to_string()),
        ).await?;
           
        let (en_title, fr_title) = match l {
            "en" => (document.title_text.clone(), translations[0].clone()),
            _ => (translations[0].clone(), document.title_text.clone())
        };

        let (en_summary, fr_summary) = match l {
            "en" => (summary.clone(), translations[1].clone()),
            _ => (translations[1].clone(), summary.clone())
        };


        let meta = InsertableMetaData {
            document_id: *document_id,
            searchable_title_en: en_title,
            searchable_title_fr: fr_title,
            author_id: document.created_by_id,
            subject_id: None,
            category_id: None,
            summary_text_en: en_summary,
            summary_text_fr: fr_summary,
            keyword_ids: None,
        };

        Ok(meta)
    }

    pub fn default(document_id: Uuid, author_id: Uuid) -> Self {
        InsertableMetaData {
            document_id,
            searchable_title_en: String::new(),
            searchable_title_fr: String::new(),
            author_id: author_id,
            subject_id: None,
            category_id: None,
            summary_text_en: String::new(),
            summary_text_fr: String::new(),
            keyword_ids: None,
        }
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
            .set(metadata)
            .get_result(&conn)?;
        Ok(v)
    }

    pub fn update_from_metadata(id: Uuid, metadata: &InsertableMetaData) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let v = diesel::update(metadata::table)
            .filter(metadata::id.eq(id))
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