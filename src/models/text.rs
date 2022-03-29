use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::{collections::BTreeMap};
use pulldown_cmark::{html, Options, Parser};
use rake::*;

use crate::{database, run_rake};
use crate::schema::texts;
use crate::errors::CustomError;


#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "texts"]
pub struct Text {
    pub id: Uuid,
    // section_id for the majority of user-entered texts. Exceptions are for texts about documents
    // Might want to make this a different data type
    pub section_id: Option<Uuid>,
    pub lang: String,
    pub content: Vec<String>,
    pub translated: Vec<bool>,
    pub keywords: serde_json::Value,
    pub machine_translation: Vec<bool>,
    pub created_at: Vec<NaiveDateTime>,
    pub created_by_id: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A struct for the text that is returned to the user (latest element of vec of changes)
pub struct LatestText {
    pub id: Uuid,
    pub section_id: Option<Uuid>,
    pub lang: String,
    pub content: String,
    pub translated: bool,
    pub keywords: serde_json::Value,
    pub machine_translation: bool,
    pub created_at: NaiveDateTime,
    pub created_by_id: Uuid,
}

impl LatestText {
    pub fn get_from(text: Text, markdown: bool) -> Self {

        let content = if markdown {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            let parser = Parser::new_ext(&text.content.last().unwrap(), options);

            let mut html_content: String = String::new();

            html::push_html(&mut html_content, parser);

            html_content
            
        } else {
            text.content.last().unwrap_or(&String::from("Unable to find content")).to_owned()
        };

        let keywords = run_rake(&content).expect("Unable to run RAKE");

        LatestText {
            id: text.id,
            section_id: text.section_id,
            lang: text.lang,
            content: content,
            translated: *text.translated.last().unwrap(),
            keywords: keywords,
            machine_translation: *text.machine_translation.last().unwrap(),
            created_at: *text.created_at.last().unwrap(),
            created_by_id: *text.created_by_id.last().unwrap(),
        }
    }
}

impl Text {
    pub fn create(text: &InsertableText) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(texts::table)
            .values(text)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_text_by_id(text_id: Uuid, lang: &str) -> Result<Text, CustomError> {
        let conn = database::connection()?;
        let text = texts::table
            .filter(texts::id.eq(text_id)
            .and(texts::lang.eq(lang)))
            .get_result(&conn)?;

        Ok(text)
    }

    pub fn get_text_by_section_id(section_id: Uuid, lang: &str) -> Result<Text, CustomError> {
        let conn = database::connection()?;
        let text = texts::table
            .filter(texts::section_id.eq(section_id)
            .and(texts::lang.eq(lang)))
            .get_result(&conn)?;

        Ok(text)
    }

    pub fn get_text_map(ids: Vec<Uuid>, lang: &str) -> Result<BTreeMap<Uuid, String>, CustomError> {
        let conn = database::connection()?;
        let texts = texts::table
            .filter(texts::id.eq_any(ids)
            .and(texts::lang.eq(lang)))
            .load::<Text>(&conn)?;

        let mut treemap = BTreeMap::new();

        for t in texts {
            // get the latest version of the text
            treemap.insert(t.id, t.content.last().unwrap().to_owned());
        };

        Ok(treemap)
    }

    pub fn update(
        text_id: Uuid, 
        content: String, 
        lang: &str,
        created_by_id:Uuid,
    ) -> Result<Self, CustomError> {
        let conn = database::connection()?;

        let mut text = Text::get_text_by_id(text_id, lang).expect("Unable to retrieve text");

        text.content.push(content);
        text.translated.push(false);
        text.machine_translation.push(false);
        text.created_by_id.push(created_by_id);
        text.created_at.push(chrono::Utc::now().naive_utc());

        let v = diesel::update(texts::table)
            .filter(texts::id.eq(text_id))
            .set(text)
            .get_result(&conn)?;
        Ok(v)
    }
}

impl From<InsertableText> for Text {
    fn from(text: InsertableText) -> Self {

        let keywords = run_rake(&text.content.last().unwrap())
            .expect("Unable to run RAKE");

        Text {
            id: Uuid::new_v4(),
            section_id: text.section_id,
            lang: text.lang,
            content: text.content,
            translated: text.translated,
            keywords: keywords,
            machine_translation: text.machine_translation,
            created_at: vec![chrono::Utc::now().naive_utc()],
            created_by_id: text.created_by_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "texts"]
pub struct InsertableText {
    pub lang: String,
    pub content: Vec<String>,
    pub translated: Vec<bool>,
    pub machine_translation: Vec<bool>,
    pub section_id: Option<Uuid>,
    pub created_by_id: Vec<Uuid>,
}

impl InsertableText {
    pub fn new_detailed(
        lang: String,
        content: String,
        translated: bool,
        machine_translation: bool,
        section_id: Option<Uuid>,
        created_by_id: Uuid
    ) -> Self {

        let content = vec![content];
        let translated = vec![translated];
        let machine_translation = vec![machine_translation];
        let created_by_id = vec![created_by_id];

        InsertableText {
            lang,
            content,
            translated,
            machine_translation,
            section_id,
            created_by_id,
        }
    }pub fn new(
        section_id: Option<Uuid>,
        lang: &str,
        content: String,
        created_by_id: Uuid,
    ) -> Self {

        let content = vec![content];
        let translated = vec![false];
        let machine_translation = vec![false];
        let created_by_id = vec![created_by_id];

        InsertableText {
            lang: lang.to_owned(),
            content,
            translated,
            machine_translation,
            section_id,
            created_by_id,
        }
    }
}