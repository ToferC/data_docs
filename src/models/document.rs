use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{BelongingToDsl, QueryDsl};
use chrono::prelude::*;
use std::collections::BTreeMap;


use crate::database;
use crate::schema::{documents, sections, texts};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text, ReadableTemplateSection, Section, ReadableSection};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
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
// A human readable Document
pub struct ReadableDocument {
    pub id: Uuid,
    pub title_text: String,
    pub purpose_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub publishable: bool,
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
        created_by: Uuid,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            lang.to_owned(), 
            raw_title_text.to_owned(),
            None,
            created_by);

        let title_text = Text::create(&insertable_name_text)?;

        let insertable_purpose_text = InsertableText::new(
            lang.to_owned(), 
            raw_purpose_text,
            None,
            created_by);

        let purpose_text = Text::create(&insertable_purpose_text)?;

        Ok(InsertableDocument {
            title_text_id: title_text.id,
            purpose_text_id: purpose_text.id,
        })
    }
}

impl Document {
    pub fn create(
        document: &InsertableDocument,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(documents::table)
            .values(document)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_readable_by_id(id: Uuid, lang: &str) -> Result<(ReadableDocument, BTreeMap<Uuid, ReadableSection>), CustomError> {
        let conn = database::connection()?;

        let document = documents::table
            .filter(documents::id.eq(id))
            .first::<Self>(&conn)?;

        let sections = Section::belonging_to(&document)
            .load::<Section>(&conn)?;

        // Get texts for document
        let mut text_ids = Vec::new();

        text_ids.push(document.title_text_id);
        text_ids.push(document.purpose_text_id);

        let texts = Text::get_text_map(text_ids, lang)?;

        let readable_document = ReadableDocument {
            id: document.id,
            title_text: texts.get(&document.title_text_id).unwrap().to_string(),
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            created_at: document.created_at,
            updated_at: document.updated_at,
            publishable: document.publishable,
        };

        // Get the ReadableSections with the data that we need to render them

        let mut readable_sections: BTreeMap<Uuid, ReadableSection> = BTreeMap::new();

        for section in sections.iter() {
            let rs = ReadableSection::get_by_id(section.id, lang)?;
            readable_sections.insert(section.id, rs);
        }

        Ok((readable_document, readable_sections))
    }

    pub fn get_all_readable(lang: &str) -> Result<Vec<ReadableDocument>, CustomError> {
        let conn = database::connection()?;

        let documents = documents::table
            .load::<Self>(&conn)?;

        let mut text_ids = Vec::new();

        // Get texts for document
        for document in documents.iter() {
            text_ids.push(document.title_text_id);
            text_ids.push(document.purpose_text_id);
        };

        let texts = Text::get_text_map(text_ids, lang)?;

        let mut readable_documents = Vec::new();

        for document in documents.iter() {
            let readable_document = ReadableDocument {
                id: document.id,
                title_text: texts.get(&document.title_text_id).unwrap().to_string(),
                purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
                created_at: document.created_at,
                updated_at: document.updated_at,
                publishable: document.publishable,
            };

            readable_documents.push(readable_document);
        };
            
        Ok(readable_documents)
    }

    pub fn get_texts(&self, lang: &str) -> Vec<Text> {
        let conn = database::connection().unwrap();

        let mut text_ids = Vec::new();
        text_ids.push(self.title_text_id);
        text_ids.push(self.purpose_text_id);

        let texts = texts::table
            .filter(texts::lang.eq(lang))
            .filter(texts::id.eq_any(text_ids))
            .load::<Text>(&conn)
            .unwrap();

        texts
    }
}