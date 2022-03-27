use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{BelongingToDsl, QueryDsl};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::errors::CustomError;
use crate::database;
use crate::schema::{template_sections, texts};
use crate::models::{Text, InsertableText, Template, ReadableSection};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, Associations, Identifiable, Clone)]
#[table_name = "template_sections"]
#[belongs_to(Template)]
pub struct TemplateSection {
    pub id: Uuid,
    pub template_id: Uuid,
    pub header_text_id: Uuid,
    pub order_number: i32,
    pub instructions_text_id: Uuid,
    pub help_text_id: Uuid,
    pub character_limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadableTemplateSection {
    pub id: Uuid,
    pub template_id: Uuid,
    pub header_text: String,
    pub order_number: i32,
    pub instructions_text: String,
    pub help_text: String,
    pub character_limit: i32,
}

impl TemplateSection {
    pub fn create(template_section: &InsertableTemplateSection) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(template_sections::table)
            .values(template_section)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_texts(&self, lang: &str) -> Vec<Text> {
        let conn = database::connection().unwrap();

        let mut text_ids = Vec::new();
        text_ids.push(self.header_text_id);
        text_ids.push(self.instructions_text_id);
        text_ids.push(self.help_text_id);

        let texts = texts::table
            .filter(texts::lang.eq(lang))
            .filter(texts::id.eq_any(text_ids))
            .load::<Text>(&conn)
            .unwrap();

        texts
    }

    pub fn get_readable_by_id(id: Uuid, lang: &str) -> Result<ReadableTemplateSection, CustomError> {
        let conn = database::connection()?;

        let template_section: TemplateSection = template_sections::table
            .filter(template_sections::id.eq(id))
            .get_result(&conn)?;

        // Get texts for template and each section
        let mut text_ids = Vec::new();

        text_ids.push(template_section.header_text_id);
        text_ids.push(template_section.instructions_text_id);
        text_ids.push(template_section.help_text_id);

        let texts = Text::get_text_map(text_ids, lang)?;

        let limit = if let Some(i) = template_section.character_limit {
            i
        } else {
            0
        };

        let readable_template_section = ReadableTemplateSection {
            header_text: texts.get(&template_section.header_text_id).unwrap().to_string(),
            instructions_text: texts.get(&template_section.instructions_text_id).unwrap().to_string(),
            help_text: texts.get(&template_section.help_text_id).unwrap().to_string(),
            order_number: template_section.order_number,
            character_limit: limit,
            id: template_section.id,
            template_id: template_section.template_id,
        };

        Ok(readable_template_section)
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "template_sections"]
pub struct InsertableTemplateSection {
    pub template_id: Uuid,
    pub header_text_id: Uuid,
    pub order_number: i32,
    pub instructions_text_id: Uuid,
    pub help_text_id: Uuid,
    pub character_limit: Option<i32>,
}

impl InsertableTemplateSection {
    pub fn new(
        template_id: Uuid,
        header_text: String,
        order_number: i32,
        instructions_text: String,
        help_text: String,
        character_limit: Option<i32>,
        lang: String,
        created_by_id: Uuid,
    ) -> Result<Self, CustomError> {

        let insertable_header_text = InsertableText::new(
            None,
            &lang, 
            header_text,
            created_by_id);

        let header_text = Text::create(&insertable_header_text)?;

        let insertable_instructions_text = InsertableText::new(
            None,
            &lang, 
            instructions_text,
            created_by_id);

        let instructions_text = Text::create(&insertable_instructions_text)?;

        let insertable_help_text = InsertableText::new(
            None,
            &lang, 
            help_text,
            created_by_id);

        let help_text = Text::create(&insertable_help_text)?;

        Ok(InsertableTemplateSection {
            template_id,
            header_text_id: header_text.id,
            order_number,
            instructions_text_id: instructions_text.id,
            help_text_id: help_text.id,
            character_limit,
        })
    }
}
