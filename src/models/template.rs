use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{BelongingToDsl, QueryDsl};
use chrono::NaiveDateTime;
use uuid::Uuid;
use inflector::Inflector;

use crate::errors::CustomError;
use crate::database;
use crate::schema::{templates, texts};
use crate::models::{Text, InsertableText, TemplateSection, ReadableTemplateSection};

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset, Queryable, Identifiable)]
/// Core data structure which to build a Document
///Intent here is that users can create template with all the elements to enable other users
/// to build a document. An example is a memo for approval. The memo has
/// * Title
/// * Issue
/// * Background
/// * Recommendation
/// And an approval section.
/// 
/// When a user is creating a memo, they will see a screen with a single markdown window for each section. They will enter 
/// the text in the markdown window and when saved, the text will be saved to the database and the structural data will be 
/// entered to recreate and manage the document.
#[table_name = "templates"]
pub struct Template {
    pub id: Uuid,
    pub name_text_id: Uuid,
    pub purpose_text_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// A human readable Template
pub struct ReadableTemplate {
    pub id: Uuid,
    pub name_text: String,
    pub name_text_id: Uuid,
    pub purpose_text: String,
    pub purpose_text_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub slug: String,
}

impl Template {
    pub fn create_with_id(
        id: Uuid,
        raw_name_text: String,
        raw_purpose_text: String,
        lang: &str,
        created_by: Uuid,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            None,
            &lang, 
            raw_name_text.to_owned(),
            created_by);

        let name_text = Text::create(&insertable_name_text)?;

        let insertable_purpose_text = InsertableText::new(
            None,
            &lang, 
            raw_purpose_text,
            created_by);

        let slug = raw_name_text.to_snake_case();

        let purpose_text = Text::create(&insertable_purpose_text)?;

        let template = Template {
            id: id,
            name_text_id: name_text.id,
            purpose_text_id: purpose_text.id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            slug: slug,
        };

        let conn = database::connection()?;

        let v = diesel::insert_into(templates::table)
            .values(template)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn create(
        template: &InsertableTemplate,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(templates::table)
            .values(template)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn update(
        template: &InsertableTemplate,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(templates::table)
            .values(template)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn get_readable_core_by_id(id: Uuid, lang: &str) -> Result<ReadableTemplate, CustomError> {
        let conn = database::connection()?;

        let template = templates::table
            .filter(templates::id.eq(id))
            .first::<Self>(&conn)?;

        // Get texts for template and each section
        let mut text_ids = Vec::new();

        text_ids.push(template.name_text_id);
        text_ids.push(template.purpose_text_id);

        let texts = Text::get_text_map(text_ids, lang)?;

        let readable_template = ReadableTemplate {
            id: template.id,
            name_text: texts.get(&template.name_text_id).unwrap().to_string(),
            name_text_id: template.name_text_id,
            purpose_text: texts.get(&template.purpose_text_id).unwrap().to_string(),
            purpose_text_id: template.purpose_text_id,
            created_at: template.created_at,
            updated_at: template.updated_at,
            slug: template.slug.to_owned(),
        };

        Ok(readable_template)
    }

    pub fn get_core_by_id(id: Uuid) -> Result<Template, CustomError> {
        let conn = database::connection()?;

        let template = templates::table
            .filter(templates::id.eq(id))
            .first::<Self>(&conn)?;

        Ok(template)
    }

    pub fn get_readable_by_id(id: Uuid, lang: &str) -> Result<(ReadableTemplate, BTreeMap<Uuid, ReadableTemplateSection>), CustomError> {
        let conn = database::connection()?;

        let template = templates::table
            .filter(templates::id.eq(id))
            .first::<Self>(&conn)?;

        let sections = TemplateSection::belonging_to(&template)
            .load::<TemplateSection>(&conn)?;

        // Get texts for template and each section
        let mut text_ids = Vec::new();

        text_ids.push(template.name_text_id);
        text_ids.push(template.purpose_text_id);

        for section in sections.iter() {
            text_ids.push(section.header_text_id);
            text_ids.push(section.instructions_text_id);
            text_ids.push(section.help_text_id);
        }

        let texts = Text::get_text_map(text_ids, lang)?;

        let readable_template = ReadableTemplate {
            id: template.id,
            name_text: texts.get(&template.name_text_id).unwrap().to_string(),
            name_text_id: template.name_text_id,
            purpose_text: texts.get(&template.purpose_text_id).unwrap().to_string(),
            purpose_text_id: template.purpose_text_id,
            created_at: template.created_at,
            updated_at: template.updated_at,
            slug: template.slug.to_owned(),
        };

        let mut readable_sections: BTreeMap<Uuid, ReadableTemplateSection> = BTreeMap::new();

        for template_section in sections.iter() {

            let limit = if let Some(i) = template_section.character_limit {
                i
            } else {
                0
            };

            let readable_template_section = ReadableTemplateSection {
                header_text: texts.get(&template_section.header_text_id).unwrap().to_string(),
                header_text_id: template_section.header_text_id,
                instructions_text: texts.get(&template_section.instructions_text_id).unwrap().to_string(),
                instructions_text_id: template_section.instructions_text_id,
                help_text: texts.get(&template_section.help_text_id).unwrap().to_string(),
                help_text_id: template_section.help_text_id,
                order_number: template_section.order_number,
                character_limit: limit,
                id: template_section.id,
                template_id: template_section.template_id,
            };

            readable_sections.insert(template_section.id, readable_template_section);
        }

        Ok((readable_template, readable_sections))
    }

    pub fn get_all_readable(lang: &str) -> Result<Vec<ReadableTemplate>, CustomError> {
        let conn = database::connection()?;

        let templates = templates::table
            .load::<Self>(&conn)?;

        let mut text_ids = Vec::new();

        // Get texts for template
        for template in templates.iter() {
            text_ids.push(template.name_text_id);
            text_ids.push(template.purpose_text_id);
        };

        let texts = Text::get_text_map(text_ids, lang)?;

        let mut readable_templates = Vec::new();

        for template in templates.iter() {
            let readable_template = ReadableTemplate {
                id: template.id,
                name_text: texts.get(&template.name_text_id).unwrap().to_string(),
                name_text_id: template.name_text_id,
                purpose_text: texts.get(&template.purpose_text_id).unwrap().to_string(),
                purpose_text_id: template.purpose_text_id,
                created_at: template.created_at,
                updated_at: template.updated_at,
                slug: template.slug.to_owned(),
            };

            readable_templates.push(readable_template);
        };
            
        Ok(readable_templates)
    }

    pub fn get_all_with_data(lang: &str) -> Result<(Vec<(Template, Vec<TemplateSection>)>, BTreeMap<Uuid, String>), CustomError> {
        let conn = database::connection()?;

        let templates = templates::table
            .load::<Self>(&conn)?;

        let sections = TemplateSection::belonging_to(&templates)
            .load::<TemplateSection>(&conn)?;

        // Get texts for template and each section
        let mut text_ids = Vec::new();
        
        for template in templates.iter() {
            text_ids.push(template.name_text_id);
            text_ids.push(template.purpose_text_id);
        };

        for section in sections.clone().into_iter() {
            text_ids.push(section.header_text_id);
            text_ids.push(section.instructions_text_id);
            text_ids.push(section.help_text_id);
        }

        let sections = sections.grouped_by(&templates);

        let texts = Text::get_text_map(text_ids, lang)?;

        let v = templates
            .into_iter()
            .zip(sections)
            .collect();

        Ok((v, texts))
    }

    pub fn get_texts(&self, lang: &str) -> Vec<Text> {
        let conn = database::connection().unwrap();

        let mut text_ids = Vec::new();
        text_ids.push(self.name_text_id);
        text_ids.push(self.purpose_text_id);

        let texts = texts::table
            .filter(texts::lang.eq(lang))
            .filter(texts::id.eq_any(text_ids))
            .load::<Text>(&conn)
            .unwrap();

        texts
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "templates"]
pub struct InsertableTemplate {
    pub name_text_id: Uuid,
    pub purpose_text_id: Uuid,
    pub slug: String,
}

impl InsertableTemplate {
    pub fn new(
        raw_name_text: String,
        raw_purpose_text: String,
        lang: String,
        created_by: Uuid,
    ) -> Result<Self, CustomError> {

        let insertable_name_text = InsertableText::new(
            None,
            &lang, 
            raw_name_text.to_owned(),
            created_by);

        let name_text = Text::create(&insertable_name_text)?;

        let insertable_purpose_text = InsertableText::new(
            None,
            &lang, 
            raw_purpose_text,
            created_by);

        let slug = raw_name_text.to_snake_case();

        let purpose_text = Text::create(&insertable_purpose_text)?;

        Ok(InsertableTemplate {
            name_text_id: name_text.id,
            purpose_text_id: purpose_text.id,
            slug: slug,
        })
    }
}