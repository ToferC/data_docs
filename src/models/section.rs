use magic_crypt::MagicCryptTrait;
use serde::{Serialize, Deserialize};
use std::str;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::{QueryDsl};
use chrono::prelude::*;
use pulldown_cmark::{html, Options, Parser};

use crate::{database, get_keyword_html, process_text_redactions, MAGIC_CRYPT};
use crate::schema::{sections};
use crate::errors::CustomError;
use crate::models::{Text, Document, TemplateSection};

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

    pub fn get_by_id(
        id: Uuid,
    ) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v: Section = sections::table
            .filter(sections::id.eq(id))
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
    pub keywords: String,
    pub lang: String,
    pub character_limit: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ReadableSection {

    pub fn get_by_id(id: Uuid, lang: &str, markdown: bool, redact: bool) -> Result<ReadableSection, CustomError> {
        let conn = database::connection()?;

        let section: Section = sections::table
            .filter(sections::id.eq(id))
            .get_result(&conn)?;

        let template_section = TemplateSection::get_readable_by_id(section.template_section_id, lang)?;

        let text = Text::get_text_by_section_id(section.id, lang)
            .expect("Unable to retrieve text");

        let decrypted_content = {

            let encrypted_content = &text.content.last().unwrap().clone();

            MAGIC_CRYPT.decrypt_base64_to_string(encrypted_content).expect("Unable to decrypt")
        };

        let processed_text = process_text_redactions(decrypted_content.clone(), redact);

        let content = if markdown {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            
            let parser = Parser::new(&processed_text);
        
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);

            html_content
            
        } else {
            decrypted_content
        };

        // get keywords from text
        let keywords = get_keyword_html(text.keywords);

        let readable_section = ReadableSection {
            id: section.id,
            header_text: template_section.header_text,
            order_number: template_section.order_number,
            instructions_text: template_section.instructions_text,
            help_text: template_section.help_text,
            text_id: text.id,
            content,
            keywords,
            lang: lang.to_string(),
            character_limit: template_section.character_limit,
            created_at: section.created_at,
            updated_at: section.updated_at,
        };

        Ok(readable_section)
    }

}
