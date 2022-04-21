pub mod models;
pub mod handlers;
pub mod database;
pub mod errors;
pub mod schema;

use std::collections::BTreeMap;
use tera::{Tera, Context};
use actix_identity::Identity;
use actix_session::Session;
use std::env;
use rake::*;
use uuid::Uuid;
use core::iter::zip;
use regex::{Regex, Captures};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use sendgrid::SGClient;

use models::{User, Template, InsertableTemplate, InsertableTemplateSection, TemplateSection,
    Document, InsertableDocument, Section, InsertableSection, InsertableText, Text};
use errors::CustomError;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate lazy_static;

const APP_NAME: &str = "Data Docs";

lazy_static! {
    // Set up MagicCrypt for encryption and decryption
    // We would want to rotate our secret key in production
    static ref MAGIC_CRYPT: magic_crypt::MagicCrypt256 = magic_crypt::new_magic_crypt!(env::var("SECRET_KEY").expect("Unable to find secret key"), 256);
}

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
        true,
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
        true,
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
        true,
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
        true,
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
        true,
    )?;

    let _issue = TemplateSection::create(&issue)?;
    let _background = TemplateSection::create(&background)?;
    let _options = TemplateSection::create(&options)?;
    let _recommendation = TemplateSection::create(&recommendation)?;

    println!("{:?}", &template);

    Ok(template)

}

// Test in constucting template and simple document
pub fn construct_demo_document(template_id: Uuid, lang: &str) -> Result<Document, CustomError> {

    let user_email = env::var("ADMIN_EMAIL").expect("Unable to retrieve ADMIN_EMAIL");

    // load user
    let user = User::find_from_email(&user_email)?;

    let (_template, template_sections) = Template::get_readable_by_id(template_id, lang)?;

    let mut ordered_template_sections = BTreeMap::new();

    for (_k, v) in template_sections {
        ordered_template_sections.insert(v.order_number, v);
    }

    let insertable_document = InsertableDocument::new(
        template_id, 
        "Why Wastewater Data is Worthwhile".to_string(),
        "To inform the minister".to_string(),
        "en",
        user.id,
        "protected_b".to_string(),
        true,
    ).unwrap();

    let document = Document::create(&insertable_document).expect("Unable to create document");

    let mut document_texts = Vec::new();

    document_texts.push(r#"Overview of wastewater surveillance data interpretation and use
Appropriate public health interpretation of wastewater surveillance data ~~depends on understanding the surveillance~~[12.1] sampling strategy and testing limitations, as well as valid data processing and analysis. Wastewater surveillance data are primarily used in three ways:

Monitoring for presence of infection within a community.
Tracking trends in infection within a community.
Screening for infections at a targeted site (e.g., building or facility) to trigger additional individual-based testing and mitigation measures. Review CDC’s guidance on targeted wastewater surveillance to use wastewater surveillance data for screening."#
        .to_string());

    document_texts.push(r#"Interpretation of wastewater surveillance data
Scientific analysis of wastewater sample in laboratory
Wastewater surveillance data ~~collected at the municipal level~~[17], when analyzed appropriately, can provide information on:

Presence of infected individuals contributing to a wastewater system.
Infection trends within the community contributing to a wastewater treatment plant (known as a “sewershed”). Sewersheds with largely transient populations, such as areas with high tourism, may provide less stable signals, which should be considered when designing the wastewater surveillance plan for public health action."#
        .to_string());

    document_texts.push(r#"Trends
Wastewater trend classification is the statistical analysis of changes in the normalized concentration of SARS-CoV-2 in wastewater (i.e., not by qualitative visual assessment). Trends in these wastewater data can be used to assess COVID-19 trends (reported and unreported) within the community contributing to the sewer system. Trends can be classified into categories based on the duration and direction of change in viral quantities. The frequency of wastewater sampling will dictate the time period for which trends can be assigned.

A benefit of trend analysis is that:

~~Data from wastewater treatment plants can be compared, despite differences in population size and wastewater volume.~~[13.1]
Trends in wastewater may be known prior to COVID-19 reported case trends, given that normalized concentration of SARS-CoV-2 in wastewater has been shown to coincide with or lead new reported cases within a sewershed by days."#
        .to_string());

    document_texts.push(r#"Using wastewater surveillance to support the COVID-19 response
    SARS-CoV-2 wastewater surveillance data can help state, tribal, local, and territorial (STLT) health departments detect, understand, and respond to the COVID-19 pandemic. Wastewater surveillance can provide an early indicator of the presence of, or trends in, COVID-19 cases in a community.
    
Wastewater surveillance complements existing COVID-19 surveillance systems and should not be interpreted alone to inform public health action. Wastewater surveillance does not provide insights into the social and behavioral factors underlying changes in transmission.

Wastewater surveillance provides:

* A pooled community sample
* Data for communities where timely COVID-19 clinical testing is underutilized or unavailable
* Data at the sewershed level, which often is smaller than a county
* Information on emergence or reemergence of infections within a community prior to case reporting
* Additional information that is not affected by certain limitations of clinical indicators, such as variability in healthcare-seeking behaviors
* SARS-CoV-2 wastewater surveillance data can be used to inform clinical testing and community mitigation strategies, such as:

* Increased testing of individuals in the affected community
* Increased public health communication about how individuals can protect themselves from COVID-19 and outreach in the affected community
* Monitoring and impact evaluation of community mitigation strategies
* When evaluating wastewater-based signals for changes in SARS-CoV-2 infection levels within a community, consider the following:

Other epidemiologic knowledge is needed, such as locations of populations at higher risk of COVID-19, to determine where to deploy clinical testing resources in sub-sewershed areas.
Lack of SARS-CoV-2 detection in wastewater alone should not be used to justify relaxing community mitigation measures.
Using sampling points upstream from wastewater treatment plants to monitor sub-sewershed infection trends requires additional work to understand the boundaries and unique characteristics of that area before it can be used for wastewater surveillance."#
        .to_string());

    for ((_key, template_section), doc_content) in zip(ordered_template_sections, document_texts) {

        let insertable_section = InsertableSection::new(
            document.id,
            template_section.id,
            user.id,
        ).unwrap();

        let section = Section::create(&insertable_section)
            .expect("Unable to create section");

        let section_text = InsertableText::new(
            Some(section.id),
            lang,
            doc_content,
            user.id,
        );

        let _final_text = Text::create(&section_text, true).expect("Unable to create text");

    };

    Ok(document)

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

    // default document_view to internal if user is signed in or otherwise changed in handler
    let document_view = match session_user.as_str() {
        "" => "internal",
        _ => "open",
    };

    ctx.insert("document_view", &document_view);

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

pub fn get_keyword_html(json: Option<serde_json::Value>) -> String {
    // get keywords from text
    let keywords: String = match json {
        Some(v) => {
            let kws: Vec<rake::KeywordScore> = serde_json::from_value(v).unwrap();
            let mut return_string = String::from("<ul>");
            for i in 0..1 {
                return_string.push_str(format!("<li>{:?}: {}</li>", kws[i].keyword, kws[i].score).as_str());
            };
            return_string.push_str("</ul>");
            return_string
        },
        None => "None".to_string(),
    };
    keywords
}

/// Find text redactions in document and format for either open (redacted) or normal reading (internal)
pub fn process_text_redactions(html_string: String, redact: bool) -> String {

    lazy_static! {
        // get text or characters between ~~ tags followed by square brackets
        // Includes newlines and characters
        static ref RE: Regex = Regex::new(r"~~(?P<text>(.*?|\n)*)~~\[(?P<act>.*?|\n)\]").unwrap();
    }

    let response_string = match redact {
        true => {
            let final_string = RE.replace_all(&html_string, |caps: &Captures| {

                let text = caps.name("text").map_or("", |m| m.as_str());

                let final_text = generate_redacted_string(text);

                format!(
                    "{}[{}]",
                    final_text, 
                    caps.name("act").map_or("", |m| m.as_str())
                )
            });
            final_string.to_string()
        },
        false => {
            let final_string = RE.replace_all(&html_string, "$text");
            final_string.to_string()
        }
    };
    
    response_string
}

fn generate_redacted_string(s: &str) -> String {

    let ns = s.clone();
    let mut result = String::new();

    for w in ns.split(" ") {
        result.push_str(&"\u{25A0}".repeat(w.len()));
        result.push_str(&" ");
    };

    result
}
