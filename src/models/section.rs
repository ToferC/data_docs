use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{QueryDsl};
use chrono::prelude::*;


use crate::database;
use crate::schema::{sections, texts};
use crate::errors::CustomError;
use crate::models::{Text, TemplateSection, Document};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Queryable, Associations, Identifiable, Clone)]
#[table_name = "sections"]
#[belongs_to(Document)]
pub struct Section {
    pub id: Uuid,
    pub document_id: Uuid,
    pub template_section_id: Uuid, // References the document section so we don't duplicate the data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: Uuid,
}

impl Section {
    pub fn create(
        section: &InsertableSection,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v = diesel::insert_into(sections::table)
            .values(section)
            .get_result(&conn)?;

        Ok(v)
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "sections"]
pub struct InsertableSection {
    pub document_id: Uuid,
    pub template_section_id: Uuid,
    pub created_by_id: Uuid,
}

impl InsertableSection {
    pub fn new(
        document_id: Uuid,
        template_section_id: Uuid,
        created_by_id: Uuid,
    ) -> Result<Self, CustomError> {

       

        Ok(InsertableSection {
            document_id,
            template_section_id,
            created_by_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// A human readable section including elements from the template
pub struct ReadableSection {
    pub id: Uuid,
    pub header_text: String,
    pub order_number: i32,
    pub instructions_text: String,
    pub help_text: String,
    pub text_id: Uuid,
    pub content: String,
    pub lang: String,
    pub character_limit: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ReadableSection {

    pub fn get_by_id(id: Uuid, lang: &str) -> Result<ReadableSection, CustomError> {
        let conn = database::connection()?;

        let section: Section = sections::table
            .filter(sections::id.eq(id))
            .get_result(&conn)?;

        let template_section = TemplateSection::get_readable_by_id(section.template_section_id, lang)?;

        let text = texts::table
            .filter(texts::section_id.eq(section.id)
            .and(texts::lang.eq(lang)))
            .first::<Text>(&conn)?;

        let readable_section = ReadableSection {
            id: section.id,
            header_text: template_section.header_text,
            order_number: template_section.order_number,
            instructions_text: template_section.instructions_text,
            help_text: template_section.help_text,
            text_id: text.id,
            content: text.content.last().unwrap_or(&String::from("Unable to find content")).to_owned(),
            lang: lang.to_string(),
            character_limit: template_section.character_limit,
            created_at: section.created_at,
            updated_at: section.updated_at,
        };

        Ok((readable_section))
    }

}