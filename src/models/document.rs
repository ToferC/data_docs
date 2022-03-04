use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{BelongingToDsl, QueryDsl};
use chrono::prelude::*;
use std::collections::BTreeMap;


use crate::database;
use crate::schema::{documents, sections, texts};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text, ReadableTemplateSection};

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

    pub fn get_by_id(id: Uuid, lang: &str) -> Result<(ReadableDocument, Vec<ReadableTemplateSection>), CustomError> {
        let conn = database::connection()?;

        let document = documents::table
            .filter(documents::id.eq(id))
            .first::<Self>(&conn)?;

        let sections = Section::belonging_to(&document)
            .load::<Section>(&conn)?;

        // Get texts for document and each section
        let mut text_ids = Vec::new();

        text_ids.push(document.title_text_id);
        text_ids.push(document.purpose_text_id);

        for section in sections.iter() {
            text_ids.push(section.header_text_id);
            text_ids.push(section.instructions_text_id);
            text_ids.push(section.help_text_id);
        }

        let texts = Text::get_text_map(text_ids, lang)?;

        let readable_document = ReadableDocument {
            id: document.id,
            title_text: texts.get(&document.title_text_id).unwrap().to_string(),
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            created_at: document.created_at,
            updated_at: document.updated_at,
            publishable: document.publishable,
        };

        let mut readable_sections = Vec::new();

        for section in sections.iter() {

            let limit = if let Some(i) = section.character_limit {
                i
            } else {
                0
            };

            let readable_template_section = ReadableTemplateSection {
                header_text: texts.get(&section.header_text_id).unwrap().to_string(),
                instructions_text: texts.get(&section.instructions_text_id).unwrap().to_string(),
                help_text: texts.get(&section.help_text_id).unwrap().to_string(),
                order_number: section.order_number,
                character_limit: limit,
                id: section.id,
                template_id: section.template_id,
            };

            readable_sections.push(readable_template_section);
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

    pub fn get_all_with_data(lang: &str) -> Result<(Vec<(Document, Vec<Section>)>, BTreeMap<Uuid, String>), CustomError> {
        let conn = database::connection()?;

        let documents = documents::table
            .load::<Self>(&conn)?;

        let sections = Section::belonging_to(&documents)
            .load::<Section>(&conn)?;

        // Get texts for document and each section
        let mut text_ids = Vec::new();
        
        for document in documents.iter() {
            text_ids.push(document.title_text_id);
            text_ids.push(document.purpose_text_id);
        };

        for section in sections.clone().into_iter() {
            text_ids.push(section.header_text_id);
            text_ids.push(section.instructions_text_id);
            text_ids.push(section.help_text_id);
        }

        let sections = sections.grouped_by(&documents);

        let texts = Text::get_text_map(text_ids, lang)?;

        let v = documents
            .into_iter()
            .zip(sections)
            .collect();

        Ok((v, texts))
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

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, Associations, Identifiable, Clone)]
#[table_name = "sections"]
#[belongs_to(Document)]
pub struct Section {
    pub id: Uuid,
    pub document_id: Uuid,
    pub template_section_id: Uuid, // References the document section so we don't duplicate the data
}
