pub mod models;
pub mod handlers;
pub mod database;
pub mod errors;
pub mod schema;

use tera::{Tera, Context};
use actix_identity::Identity;
use actix_session::Session;
use std::env;
use rake::*;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use sendgrid::SGClient;

use models::{User, Template, InsertableTemplate, InsertableTemplateSection, TemplateSection};
use errors::CustomError;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

const APP_NAME: &str = "Data Docs";

#[derive(Clone, Debug)]
pub struct AppData {
    pub tmpl: Tera,
    pub mail_client: SGClient,
}

// Test in constucting template and simple document
pub fn construct_demo_template() -> Result<Template, CustomError> {

    let user_email = env::var("ADMIN_EMAIL").expect("Unable to retrieve ADMIN_EMAIL");

    // load user
    let user = User::find_from_email(&user_email)?;

    let template = InsertableTemplate::new(
        "Demo Template".to_string(),
        "Demo Purpose".to_string(),
        "en".to_string(),
        user.id,
    )?;

    let template = Template::create(&template)?;

    let issue = InsertableTemplateSection::new(
        template.id,
        "Issue".to_string(),
        0,
        "Highlight the key issues in less than five bullets.".to_string(),
        "Complete the document to the best of your ability".to_string(),
        None,
        "en".to_string(),
        user.id,
    )?;

    let background = InsertableTemplateSection::new(
        template.id,
        "Background".to_string(),
        1,
        "Explain the relevant information to provide general understanding.".to_string(),
        "Complete the document to the best of your ability".to_string(),
        None,
        "en".to_string(),
        user.id,
    )?;

    let options = InsertableTemplateSection::new(
        template.id,
        "Options".to_string(),
        2,
        "Outline options available and pros and cons".to_string(),
        "Complete the document to the best of your ability".to_string(),
        None,
        "en".to_string(),
        user.id,
    )?;

    let recommendation = InsertableTemplateSection::new(
        template.id,
        "Recommendation".to_string(),
        3,
        "Outline recommendations".to_string(),
        "Complete the document to the best of your ability".to_string(),
        None,
        "en".to_string(),
        user.id,
    )?;

    let _issue = TemplateSection::create(&issue)?;
    let _background = TemplateSection::create(&background)?;
    let _options = TemplateSection::create(&options)?;
    let _recommendation = TemplateSection::create(&recommendation)?;

    println!("{:?}", &template);

    Ok(template)

}

/// Generate context, session_user, role and node_names from id and lang
pub fn generate_basic_context(
    id: Identity,
    lang: &str,
    path: &str,
) -> (Context, String, String, String) 
{    
    let mut ctx = Context::new();

    // Get session data and add to context
    let (session_user, role) = extract_identity_data(&id);
    ctx.insert("session_user", &session_user);
    ctx.insert("role", &role);

    let validated_lang = match lang {
        "fr" => "fr",
        "en" => "en",
        _ => "en",
    };

    ctx.insert("lang", &validated_lang);
    ctx.insert("path", &path);

    (ctx, session_user, role, lang.to_owned())
}

pub fn extract_session_data(session: &Session) -> (String, String) {

    let role_data = session.get::<String>("role").expect("Unable to get role from cookie");

    let role = match role_data {
        Some(r) => r,
        None => "".to_string(),
    };

    let user_data = session.get::<String>("user_name").expect("Unable to get user_name from cookie");

    let session_user = match user_data {
        Some(u) => u,
        None => "".to_string(),
    };

    println!("{}-{}", &session_user, &role);

    (session_user, role)
}

pub fn extract_identity_data(id: &Identity) -> (String, String) {

    let id_data = id.identity();

    let session_user = match id_data {
        Some(u) => u,
        None => "".to_string(),
    };

    let user = models::User::find_slim_from_slug(&session_user);

    let role = match user {
        Ok(u) => u.role,
        _ => "".to_string()
    };

    println!("{}-{}", &session_user, &role);

    (session_user, role)
}

/// Generate context, session_user and role from id and lang
pub fn generate_email_context(
    id: Identity,
    lang: &str,
    path: &str,) -> (Context, String, String, String) 
{    
let mut ctx = Context::new();

// Get session data and add to context
let (session_user, role) = extract_identity_data(&id);
ctx.insert("session_user", &session_user);
ctx.insert("role", &role);

let validated_lang = match lang {
    "fr" => "fr",
    "en" => "en",
    _ => "en",
};

ctx.insert("lang", &validated_lang);
ctx.insert("path", &path);

(ctx, session_user, role, lang.to_owned())
}

pub fn generate_unique_code(mut characters: usize, dashes: bool) -> String {

    if characters > 64 {
        characters = 64;
    };

    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(characters)
        .map(char::from)
        .collect();

    if dashes {
        for i in 0..rand_string.len() + rand_string.len() / 4 {
            if i > 2 && i % 4 == 0 {
                rand_string.insert(i, '-');
            }
        }
    };

    rand_string
}

// Run Rake on Text and return JSON struct of keywords
pub fn run_rake(content: &str) -> Result<serde_json::Value, CustomError> {
    let sw = StopWords::from_file("stop_word_list.txt")
        .expect("Unable to load stopwords");

    let r = Rake::new(sw);
    let keywords = r.run(&content);

    Ok(serde_json::to_value(&keywords).unwrap())
}
