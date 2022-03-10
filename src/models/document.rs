use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{BelongingToDsl, QueryDsl};
use chrono::prelude::*;
use std::collections::BTreeMap;


use crate::database;
use crate::schema::{documents, template_sections, texts};
use crate::errors::CustomError;
use crate::models::{InsertableText, Text, TemplateSection,
    ReadableTemplateSection, User, Section, ReadableSection};

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
    pub publishable: bool,
    pub created_by_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// A human readable Document
pub struct ReadableDocument {
    pub id: Uuid,
    pub template_id: Uuid,
    pub title_text: String,
    pub purpose_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub publishable: bool,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "documents"]
pub struct InsertableDocument {
    pub template_id: Uuid,
    pub title_text_id: Uuid,
    pub purpose_text_id: Uuid,
    pub created_by_id: Uuid,
}

impl InsertableDocument {
    pub fn new(
        template_id: Uuid,
        raw_title_text: String,
        raw_purpose_text: String,
        lang: &str,
        created_by_id: Uuid,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            None,
            &lang, 
            raw_title_text.to_owned(),
            created_by_id);

        let title_text = Text::create(&insertable_name_text)?;

        let insertable_purpose_text = InsertableText::new(
            None,
            &lang, 
            raw_purpose_text,
            created_by_id);

        let purpose_text = Text::create(&insertable_purpose_text)?;

        Ok(InsertableDocument {
            template_id,
            title_text_id: title_text.id,
            purpose_text_id: purpose_text.id,
            created_by_id: created_by_id,
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

        let user_email = User::find_email_from_id(document.created_by_id)?;

        let readable_document = ReadableDocument {
            id: document.id,
            template_id: document.template_id,
            title_text: texts.get(&document.title_text_id).unwrap().to_string(),
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            created_at: document.created_at,
            updated_at: document.updated_at,
            publishable: document.publishable,
            created_by: user_email,
        };

        // Get the ReadableSections with the data that we need to render them

        let mut readable_sections: BTreeMap<Uuid, ReadableSection> = BTreeMap::new();

        for section in sections.iter() {
            let rs = ReadableSection::get_by_id(section.id, lang)?;
            readable_sections.insert(section.id, rs);
        }

        Ok((readable_document, readable_sections))
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
            purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
            created_at: document.created_at,
            updated_at: document.updated_at,
            publishable: document.publishable,
            created_by: user_email,
        };

        // Get the ReadableSections with the data that we need to render them

        let mut readable_sections: BTreeMap<Uuid, ReadableTemplateSection> = BTreeMap::new();

        for section in sections.iter() {
            let rs = TemplateSection::get_readable_by_id(section.id, lang)?;
            readable_sections.insert(section.id, rs);
        }

        Ok((readable_document, readable_sections))
    }

    pub fn get_all_readable(lang: &str) -> Result<Vec<ReadableDocument>, CustomError> {
        let conn = database::connection()?;

        let documents = documents::table
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
                purpose_text: texts.get(&document.purpose_text_id).unwrap().to_string(),
                created_at: document.created_at,
                updated_at: document.updated_at,
                publishable: document.publishable,
                created_by: users.get(&document.created_by_id).unwrap().to_string(),
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