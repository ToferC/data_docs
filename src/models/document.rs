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
    ReadableTemplateSection, User, Section, ReadableSection,
    MetaData, InsertableMetaData};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "documents"]
pub struct Document {
    pub id: Uuid,
    pub template_id: Uuid,
    pub title_text_id: Uuid,
    pub purpose_text_id: Uuid,
    // pub approvals: Option<Vec<Uuid>>, // Replace with Approvals
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub security_classification: String,
    pub published: bool,
    pub created_by_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// A human readable Document
pub struct ReadableDocument {
    pub id: Uuid,
    pub template_id: Uuid,
    pub title_text: String,
    pub title_text_id: Uuid,
    pub purpose_text: String,
    pub purpose_text_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub security_classification: String,
    pub published: bool,
    pub created_by: String,
    pub created_by_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "documents"]
pub struct InsertableDocument {
    pub template_id: Uuid,
    pub title_text_id: Uuid,
    pub purpose_text_id: Uuid,
    pub created_by_id: Uuid,
    pub security_classification: String,
}

impl InsertableDocument {
    pub fn new(
        template_id: Uuid,
        raw_title_text: String,
        raw_purpose_text: String,
        lang: &str,
        created_by_id: Uuid,
        security_classification: String,
        machine_translate: bool,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            None,
            &lang, 
            raw_title_text.to_owned(),
            created_by_id);

        let title_text = Text::create(&insertable_name_text, machine_translate)?;

        let insertable_purpose_text = InsertableText::new(
            None,
            &lang, 
            raw_purpose_text,
            created_by_id);

        let purpose_text = Text::create(&insertable_purpose_text, machine_translate)?;

        Ok(InsertableDocument {
            template_id,
            title_text_id: title_text.id,
            purpose_text_id: purpose_text.id,
            created_by_id: created_by_id,
            security_classification,
        })
    }
}

impl Document {
    pub fn create(
        document: &InsertableDocument,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v: Document = diesel::insert_into(documents::table)
            .values(document)
            .get_result(&conn)?;

        let default_meta = InsertableMetaData::default(v.id, document.created_by_id);

        let _created_meta = MetaData::create(&default_meta)?;

        Ok(v)
    }

    pub fn get_by_id(id: Uuid) -> Result<Document, CustomError> {
        let conn = database::connection()?;

        let document = documents::table
            .filter(documents::id.eq(id))
            .first::<Self>(&conn)?;

        Ok(document)
    }

    pub fn get_readable_core_by_id(id: Uuid, lang: &str, markdown: bool, redact: bool) -> Result<ReadableDocument, CustomError> {
        let conn = database::connection()?;

        let document = documents::table
            .filter(documents::id.eq(id))
            .first::<Self>(&conn)?;

        // Get texts for document
        let mut text_ids = Vec::new();

        text_ids.push(document.title_text_id);
        text_ids.push(document.purpose_text_id);

        // Todo -> enable markdown for purpose text
        
        let raw_texts = Text::get_text_map(text_ids, lang)?;

        let mut texts: BTreeMap<Uuid, String> = BTreeMap::new();

        // Process and redact purpose text if necessary

        for (k, v) in raw_texts {
            
            let processed_text = process_text_redactions(v.clone(), redact);
    
            let content = if markdown {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_TABLES);
                
                let parser = Parser::new(&processed_text);
            
                let mut html_content = String::new();
                html::push_html(&mut html_content, parser);
    
                html_content
                
            } else {
                processed_text
            };
            texts.insert(k, content);

        }

        let user_email = User::find_email_from_id(document.created_by_id)?;

        let readable_document = ReadableDocument {
            id: document.id,
            template_id: document.template_id,
            title_text: texts.get(&document.title_text_id).unwrap().to_string(),
            title_text_id: document.title_text_id,
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            purpose_text_id: document.purpose_text_id,
            created_at: document.created_at,
            updated_at: document.updated_at,
            published: document.published,
            security_classification: document.security_classification.to_string(),
            created_by: user_email,
            created_by_id: document.created_by_id,
        };

        Ok(readable_document)
    }

    pub fn get_readable_sections_by_id(id: Uuid, lang: &str, markdown: bool, redact: bool) -> Result<BTreeMap<Uuid, ReadableSection>, CustomError> {
        let conn = database::connection()?;

        let sections = sections::table
            .filter(sections::document_id.eq(id))
            .load::<Section>(&conn)?;

        // Get the ReadableSections with the data that we need to render them

        let mut readable_sections: BTreeMap<Uuid, ReadableSection> = BTreeMap::new();

        for section in sections.iter() {
            let rs = ReadableSection::get_by_id(section.id, lang, markdown, redact)?;
            readable_sections.insert(section.id, rs);
        }

        Ok(readable_sections)
    }

    pub fn get_all_readable_by_id(id: Uuid, lang: &str, markdown: bool, redact: bool) -> Result<(ReadableDocument, BTreeMap<Uuid, ReadableSection>), CustomError> {

        let document = Document::get_readable_core_by_id(id, lang, markdown, redact)?;

        let readable_sections: BTreeMap<Uuid, ReadableSection> = Document::get_readable_sections_by_id(
            id, lang, markdown, redact)?;

        Ok((document, readable_sections))
    }

    pub fn get_readable_plus_template_sections_by_id(id: Uuid, lang: &str) -> Result<(ReadableDocument, BTreeMap<Uuid, ReadableTemplateSection>), CustomError> {
        let conn = database::connection()?;

        let document = documents::table
            .filter(documents::id.eq(id))
            .first::<Self>(&conn)?;

        let sections = template_sections::table
            .filter(template_sections::template_id.eq(document.template_id))
            .load::<TemplateSection>(&conn)?;

        // Get texts for document
        let mut text_ids = Vec::new();

        text_ids.push(document.title_text_id);
        text_ids.push(document.purpose_text_id);

        let texts = Text::get_text_map(text_ids, lang)?;

        let user_email = User::find_email_from_id(document.created_by_id)?;

        let readable_document = ReadableDocument {
            id: document.id,
            template_id: document.template_id,
            title_text: texts.get(&document.title_text_id).unwrap().to_string(),
            title_text_id: document.title_text_id,
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            purpose_text_id: document.purpose_text_id,
            created_at: document.created_at,
            updated_at: document.updated_at,
            security_classification: document.security_classification.to_string(),
            published: document.published,
            created_by: user_email,
            created_by_id: document.created_by_id,
        };

        // Get the ReadableSections with the data that we need to render them

        let mut readable_sections: BTreeMap<Uuid, ReadableTemplateSection> = BTreeMap::new();

        for section in sections.iter() {
            let rs = TemplateSection::get_readable_by_id(section.id, lang)?;
            readable_sections.insert(section.id, rs);
        }

        Ok((readable_document, readable_sections))
    }

    pub fn get_all_readable(lang: &str, document_view: &str) -> Result<Vec<ReadableDocument>, CustomError> {
        let conn = database::connection()?;

        // External view only gets published documents
        let documents = match document_view {
            "internal" => documents::table
                .load::<Self>(&conn)?,
            _ => documents::table
                .filter(documents::published.eq(true))
                .load::<Self>(&conn)?,
        };

        let mut text_ids = Vec::new();
        let mut user_ids = Vec::new();

        // Get texts for document
        for document in documents.iter() {
            text_ids.push(document.title_text_id);
            text_ids.push(document.purpose_text_id);
            user_ids.push(document.created_by_id);
        };

        let texts = Text::get_text_map(text_ids, lang)?;
        let users = User::get_user_email_map(user_ids)?;

        let mut readable_documents = Vec::new();

        for document in documents.iter() {
            let readable_document = ReadableDocument {
                id: document.id,
                template_id: document.template_id,
                title_text: texts.get(&document.title_text_id).unwrap().to_string(),
                title_text_id: document.title_text_id,
                purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
                purpose_text_id: document.purpose_text_id,
                created_at: document.created_at,
                updated_at: document.updated_at,
                security_classification: document.security_classification.to_string(),
                published: document.published,
                created_by: users.get(&document.created_by_id).unwrap().to_string(),
                created_by_id: document.created_by_id,
            };

            readable_documents.push(readable_document);
        };
            
        Ok(readable_documents)
    }

    pub fn get_readable_by_created_by(created_by: Uuid, lang: &str) -> Result<Vec<ReadableDocument>, CustomError> {
        let conn = database::connection()?;

        let documents = documents::table
            .filter(documents::created_by_id.eq(created_by))
            .load::<Self>(&conn)?;

        let mut text_ids = Vec::new();
        let mut user_ids = Vec::new();

        // Get texts for document
        for document in documents.iter() {
            text_ids.push(document.title_text_id);
            text_ids.push(document.purpose_text_id);
            user_ids.push(document.created_by_id);
        };

        let texts = Text::get_text_map(text_ids, lang)?;
        let users = User::get_user_email_map(user_ids)?;

        let mut readable_documents = Vec::new();

        for document in documents.iter() {
            let readable_document = ReadableDocument {
                id: document.id,
                template_id: document.template_id,
                title_text: texts.get(&document.title_text_id).unwrap().to_string(),
                title_text_id: document.title_text_id,
                purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
                purpose_text_id: document.purpose_text_id,
                created_at: document.created_at,
                updated_at: document.updated_at,
                security_classification: document.security_classification.to_string(),
                published: document.published,
                created_by: users.get(&document.created_by_id).unwrap().to_string(),
                created_by_id: document.created_by_id,
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

    pub fn update(document: &Document) -> Result<Self, CustomError> {
        let conn = database::connection()?;
        let v = diesel::update(documents::table)
            .filter(documents::id.eq(document.id))
            .set(document)
            .get_result(&conn)?;
        Ok(v)
    }
}