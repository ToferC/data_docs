use std::collections::BTreeMap;

use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::models::{Section, User, Template, Document, InsertableDocument, InsertableSection,
    InsertableText, Text};
use super::DocumentForm;
use crate::errors::CustomError;

#[get("/{lang}/document_index/{document_view}")]
pub async fn document_index(
    data: web::Data<AppData>,
    web::Path((lang, mut document_view)): web::Path<(String, String)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() &&
        document_view == "internal" {
            // send to external view
            return HttpResponse::Found().header(
                "Location", 
                format!("/{}/document_index/open", lang)).finish()
    };

    let documents_data = Document::get_all_readable(&lang, &document_view)
        .expect("Unable to load templates");

    ctx.insert("documents", &documents_data);
    ctx.insert("document_view", &document_view);

    let rendered = data.tmpl.render("documents/document_index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/document/{document_id}/{document_view}")]
pub async fn get_document(
    data: web::Data<AppData>,
    web::Path((lang, document_id, document_view)): web::Path<(String, Uuid, String)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() &&
        document_view == "internal" {
            // send to external view
            return HttpResponse::Found().header("Location", format!("/{}/document/{}/open", lang, document_id)).finish()
    };

    let redact = match document_view.as_str() {
        "internal" => false,
        _ => true,
    };

    let (document, sections) = Document::get_readable_by_id(
        document_id, &lang, true, redact).expect("Unable to retrieve text");

    let mut ordered_sections = BTreeMap::new();

    for (_k, v) in sections {
        ordered_sections.insert(v.order_number, v);
    };

    ctx.insert("document", &document);
    ctx.insert("sections", &ordered_sections);
    ctx.insert("document_view", &document_view);

    let rendered = data.tmpl.render("documents/document.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/create_document_core/{template_id}")]
pub async fn create_document_core(
    data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let (template, sections) = Template::get_readable_by_id(template_id, &lang).expect("Unable to load template");

        let mut ordered_sections = BTreeMap::new();

        for (_k, v) in sections {
            ordered_sections.insert(v.order_number, v);
        }

        ctx.insert("template", &template);
        ctx.insert("sections", &ordered_sections);

        let rendered = data.tmpl.render("documents/create_document_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/save_document_core/{template_id}")]
// Put and create core of document that we need and redirect to page where
// user can create the document sections with the linkable id.
pub async fn save_document_core(
    _data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    form: web::Form<DocumentForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (_ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        return err.error_response()
    } else {

        // validate authorized to edit document
        let raw_title = form.title.trim().to_string();
        let raw_purpose = form.purpose.trim().to_string();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // create document
        let document = InsertableDocument::new(
            template_id,
            raw_title,
            raw_purpose,
            &lang,
            user.id,
        ).expect("Unable to generate insertable_document");

        let document = Document::create(&document)
            .expect("Unable to create document");

        // get template sections to create document sections
        let (_template, template_sections) = Template::get_readable_by_id(template_id, &lang)
            .expect("Unable to load sections");

        for (k, v) in template_sections {
            // create document section
            let document_section = InsertableSection::new(
                document.id,
                k,
                user.id,
            ).expect("Unable to generate insertable_section");

            let section = Section::create(&document_section)
                .expect("Unable to create document section");

            // create document section text
            let default_text = InsertableText::new(
                Some(section.id),
                "en",
                v.help_text.clone(),
                user.id,
            );

            let _text = Text::create(&default_text)
                .expect("Unable to create text");
        }

        return HttpResponse::Found().header("Location", format!("/{}/edit_document_sections/{}", &lang, &document.id)).finish()
    }
}

#[get("/{lang}/edit_document_sections/{document_id}")]
pub async fn edit_document_sections(
    data: web::Data<AppData>,
    web::Path((lang, document_id)): web::Path<(String, Uuid)>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let (document, sections) = Document::get_readable_by_id(document_id, &lang, false, false)
            .expect("Unable to load document");

        let mut ordered_sections = BTreeMap::new();

        for (_k, v) in sections {
            ordered_sections.insert(v.order_number, v);
        }

        ctx.insert("document", &document);
        ctx.insert("sections", &ordered_sections);

        let rendered = data.tmpl.render("documents/edit_document_sections.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/switch_document_published/{document_id}/{document_view}")]
pub async fn switch_document_published(
    _data: web::Data<AppData>,
    web::Path((lang, document_id, document_view)): web::Path<(String, Uuid, String)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (_ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() ||
        role != "admin".to_string() &&
        document_view == "internal" {
            // send to external view
            return HttpResponse::Found().header("Location", format!("/{}/document/{}/open", lang, document_id)).finish()
    };

    let redact = match document_view.as_str() {
        "internal" => false,
        _ => true,
    };

    let mut document = Document::get_by_id(
        document_id, &lang, true, redact).expect("Unable to retrieve text");

    match document.published {
        false => document.published = true,
        true => document.published = false,
    };

    Document::update(&document)
        .expect("Unable to update document");

    HttpResponse::Found().header(
        "Location",
        format!("/{}/document/{}/{}", lang, document_id, &document_view)).finish()
}