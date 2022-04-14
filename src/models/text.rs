use magic_crypt::MagicCryptTrait;
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use std::{collections::BTreeMap};
use std::str;
use pulldown_cmark::{html, Options, Parser};
use deepl_api::{DeepL, TranslatableTextList};

use crate::{database, run_rake, get_keyword_html, process_text_redactions, MAGIC_CRYPT};
use crate::schema::texts;
use crate::errors::CustomError;


#[derive(Debug, Serialize, Deserialize, AsChangeset, Queryable, Insertable, Clone)]
#[table_name = "texts"]
pub struct Text {
    pub id: Uuid,
    // section_id for the majority of user-entered texts. Exceptions are for texts about documents
    // Might want to make this a different data type
    pub section_id: Option<Uuid>,
    pub lang: String,
    pub content: Vec<String>,
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
            
            MAGIC_CRYPT.decrypt_base64_to_string(encrypted_content).expect("Unable to decrypt string")
        };

        let processed_text = process_text_redactions(decrypted_content.clone(), redact);

        let content = if markdown {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            let parser = Parser::new_ext(&processed_text, options);

            let mut html_content: String = String::new();

            html::push_html(&mut html_content, parser);

            html_content
            
        } else {
            decrypted_content
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

        let translation_lang = match text.lang.as_str() {
            "en" => "fr",
            _ => "en",
        };

        let mut translated_text = v.clone();

        translated_text.lang = translation_lang.to_string();

        let encrypted_content = MAGIC_CRYPT.encrypt_str_to_base64("default_translation_traduction_par_defaut");

        translated_text.content = vec![encrypted_content];

        let _t: Text = diesel::insert_into(texts::table)
            .values(&translated_text)
            .get_result(&conn)?;

        Ok(v)
    }

    pub fn update_or_create(text: &InsertableText) -> Result<Self, CustomError> {

        let conn = database::connection()?;

        let v: Text = diesel::insert_into(texts::table)
            .values(text)
            .on_conflict((texts::id, texts::lang))
            .do_update()
            .set(text)
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
                let encrypted_content = &t.content.last().unwrap().clone();
                
                MAGIC_CRYPT.decrypt_base64_to_string(encrypted_content).expect("Unable to decrypt string")
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
        machine_translation: bool,
    ) -> Result<Self, CustomError> {
        let conn = database::connection()?;

        let mut text = Text::get_text_by_id(text_id, lang).expect("Unable to retrieve text");

        if text.section_id != None {
            text.keywords = Some(run_rake(&content).unwrap());
        };

        let encrypted_content = MAGIC_CRYPT.encrypt_str_to_base64(content);


        text.content.push(encrypted_content);
        text.translated.push(false);
        text.machine_translation.push(machine_translation);
        text.created_by_id.push(created_by_id);
        text.created_at.push(chrono::Utc::now().naive_utc());


        let v = diesel::update(texts::table)
            .filter(texts::id.eq(text_id)
            .and(texts::lang.eq(lang)))
            .set(text)
            .get_result(&conn)?;
        Ok(v)
    }

    pub async fn machine_translate_text(&self, user_id: Uuid) -> Result<Self, CustomError> {
        // goes through Text and sends content Vec<String> to DEEPL for translation if translate_all == true
        // otherwise, translates only the last content string
        let key = match std::env::var("DEEPL_API_KEY") {
            Ok(val) if val.len() > 0 => val,
            _ => {
                eprintln!("Error: no DEEPL_API_KEY found. Please provide your API key in this environment variable.");
                std::process::exit(1);
            }
        };

        let deepl = DeepL::new(key);

        let mut source = "EN".to_string();
        let mut target = "FR".to_string();

        let translate_lang = match self.lang.as_str() {
            "en" => {
                "fr".to_string()
            },
            "fr" => {
                source = "FR".to_string();
                target = "EN".to_string();
                "en".to_string()
            },
            _ => {
                "fr".to_string()
            },
        };

        // Set up struct for DEEPL translation
        let translatable_text = TranslatableTextList {
            source_language: Some(source),
            target_language: target,
            texts: vec![self.content.last().unwrap().clone()],
        };

        // Send to API
        let translated = deepl.translate(None, translatable_text)
            .await
            .expect("Unable to return translations");

        let mut translated_strings = Vec::new();

        for t in translated {
            translated_strings.push(t.text);
        };

        let v = Text::update(
            self.id,
            translated_strings.last().unwrap().to_string(),
            &translate_lang,
            user_id,
            true,
        ).expect("Unable to update translated Text");

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

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "texts"]
pub struct InsertableText {
    pub lang: String,
    pub content: Vec<String>,
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
        created_by_id: Uuid,
    ) -> Self {
        
        let keywords = run_rake(&content)
            .expect("Unable to get keywords");

        let encrypted_content = MAGIC_CRYPT.encrypt_str_to_base64(content);

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
            
        let encrypted_content = MAGIC_CRYPT.encrypt_str_to_base64(content);

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