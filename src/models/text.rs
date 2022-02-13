use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::{collections::BTreeMap};

use crate::database;
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
    pub content: String,
    pub translated: bool,
    pub machine_translation: bool,
    pub created_at: NaiveDateTime,
}

impl Text {
    pub fn create(text: &InsertableText) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(texts::table)
            .values(text)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_text_map(ids: Vec<Uuid>, lang: &str) -> Result<BTreeMap<Uuid, String>, CustomError> {
        let conn = database::connection()?;
        let texts = texts::table
            .filter(texts::id.eq_any(ids)
            .and(texts::lang.eq(lang)))
            .load::<Text>(&conn)?;

        let mut treemap = BTreeMap::new();

        for t in texts {
            treemap.insert(t.id, t.content);
        };

        Ok(treemap)
    }
}

impl From<InsertableText> for Text {
    fn from(text: InsertableText) -> Self {
        Text {
            id: Uuid::new_v4(),
            section_id: text.section_id,
            lang: text.lang,
            content: text.content,
            translated: false,
            machine_translation: false,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "texts"]
pub struct InsertableText {
    pub lang: String,
    pub content: String,
    pub translated: bool,
    pub machine_translation: bool,
    pub section_id: Option<Uuid>,
}

impl InsertableText {
    pub fn new_detailed(
        lang: String,
        content: String,
        translated: bool,
        machine_translation: bool,
        section_id: Option<Uuid>,
    ) -> Self {
        InsertableText {
            lang,
            content,
            translated,
            machine_translation,
            section_id,
        }
    }pub fn new(
        lang: String,
        content: String,
        section_id: Option<Uuid>,
    ) -> Self {
        InsertableText {
            lang,
            content,
            translated: false,
            machine_translation: false,
            section_id,
        }
    }
}