use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::{collections::BTreeMap};
use std::io::{Read, Write};
use std::str;
use pulldown_cmark::{html, Options, Parser};

use crate::{database, run_rake, get_keyword_html, process_text_redactions};
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
    pub content: Vec<Vec<u8>>,
    pub keywords: Option<serde_json::Value>,
    pub translated: Vec<bool>,
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
    pub keywords: String,
    pub translated: bool,
    pub machine_translation: bool,
    pub created_at: NaiveDateTime,
    pub created_by_id: Uuid,
}

impl LatestText {
    pub fn get_from(text: Text, markdown: bool, redact: bool) -> Self {

        let decrypted_content = {
            let encrypted_content = &text.content.last().unwrap().clone()[..];
            
            let decryptor = match age::Decryptor::new(encrypted_content)
                .expect("Unable to create decryptor") {
                    age::Decryptor::Passphrase(d) => d,
                    _ => unreachable!(),
            };

            let mut decrypted = Vec::new();
            let mut reader = decryptor.decrypt(&age::secrecy::Secret::new(std::env::var("SECRET_KEY").unwrap()), None)
                .expect("Unable to create reader");
            
            reader.read_to_end(&mut decrypted).expect("Unable to read and decrypt");

            str::from_utf8(&decrypted).unwrap().to_string()
        };

        let processed_text = process_text_redactions(decrypted_content, redact);

        let content = if markdown {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            let parser = Parser::new_ext(&processed_text, options);

            let mut html_content: String = String::new();

            html::push_html(&mut html_content, parser);

            html_content
            
        } else {
            processed_text
        };

        // get keywords from text
        let keywords = get_keyword_html(text.keywords);
        
        LatestText {
            id: text.id,
            section_id: text.section_id,
            lang: text.lang,
            content: content,
            keywords,
            translated: *text.translated.last().unwrap(),
            machine_translation: *text.machine_translation.last().unwrap(),
            created_at: *text.created_at.last().unwrap(),
            created_by_id: *text.created_by_id.last().unwrap(),
        }
    }
}

impl Text {
    pub fn create(text: &InsertableText) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v: Text = diesel::insert_into(texts::table)
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

            let decrypted_content = {
                let encrypted_content = &t.content.last().unwrap().clone()[..];
            
                let decryptor = match age::Decryptor::new(encrypted_content)
                    .expect("Unable to create decryptor") {
                        age::Decryptor::Passphrase(d) => d,
                        _ => unreachable!(),
                };
    
                let mut decrypted = Vec::new();
                let mut reader = decryptor.decrypt(&age::secrecy::Secret::new(std::env::var("SECRET_KEY").unwrap()), None)
                    .expect("Unable to create reader");
                
                reader.read_to_end(&mut decrypted).expect("Unable to read and decrypt");
    
                str::from_utf8(&decrypted).unwrap().to_string()
            };

            // get the latest version of the text
            treemap.insert(t.id, decrypted_content);
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

        let encrypted_content = {
            let encryptor = age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(std::env::var("SECRET_KEY").unwrap()));
            let mut encrypted = Vec::new();
            let mut writer = encryptor.wrap_output(&mut encrypted)
                .expect("Unable to create writer");

            writer.write_all(content.as_ref()).expect("Unable to encrypt content");
            writer.finish().unwrap();

            encrypted
        };

        if text.section_id != None {
            text.keywords = Some(run_rake(&content).unwrap());
        };

        text.content.push(encrypted_content);
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

        let keywords: Option<serde_json::Value> = match text.section_id {
            Some(_c) => Some(text.keywords),
            None => None,
        };

        Text {
            id: Uuid::new_v4(),
            section_id: text.section_id,
            lang: text.lang,
            content: text.content,
            keywords,
            translated: text.translated,
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
    pub content: Vec<Vec<u8>>,
    pub keywords: serde_json::Value,
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
        
        let keywords = run_rake(&content)
            .expect("Unable to get keywords");

        let encrypted_content = {
            let encryptor = age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(std::env::var("SECRET_KEY").unwrap()));
            let mut encrypted = Vec::new();
            let mut writer = encryptor.wrap_output(&mut encrypted)
                .expect("Unable to create writer");

            writer.write_all(content.as_ref()).expect("Unable to encrypt content");
            writer.finish().unwrap();

            encrypted
        };

        let content = vec![encrypted_content];
        let translated = vec![translated];
        let machine_translation = vec![machine_translation];
        let created_by_id = vec![created_by_id];


        InsertableText {
            lang,
            content,
            keywords,
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

        let keywords = run_rake(&content)
            .expect("Unable to get keywords");
            
        let encrypted_content = {
            let encryptor = age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(std::env::var("SECRET_KEY").unwrap()));
            let mut encrypted = Vec::new();
            let mut writer = encryptor.wrap_output(&mut encrypted)
                .expect("Unable to create writer");

            writer.write_all(content.as_ref()).expect("Unable to encrypt content");
            writer.finish().unwrap();

            encrypted
        };

        let content = vec![encrypted_content];
        let translated = vec![false];
        let machine_translation = vec![false];
        let created_by_id = vec![created_by_id];

        InsertableText {
            lang: lang.to_owned(),
            content,
            keywords,
            translated,
            machine_translation,
            section_id,
            created_by_id,
        }
    }
}