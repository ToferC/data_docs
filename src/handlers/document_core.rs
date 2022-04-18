use std::collections::BTreeMap;

use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::models::{Section, User, Document, Template, InsertableDocument, InsertableSection,
    InsertableText, Text};
use super::DocumentForm;
use crate::errors::CustomError;

#[get("/{lang}/get_document_core/{document_id}/{document_view}")]
pub async fn get_document_core(
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

    let document = Document::get_readable_core_by_id(
        document_id, &lang, true, redact).expect("Unable to retrieve text");

    ctx.insert("document_core", &document);
    ctx.insert("document_view", &document_view);

    let rendered = data.tmpl.render("documents/document_core.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/create_document_core_form/{template_id}")]
pub async fn create_document_core_form(
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

        let rendered = data.tmpl.render("document_core/create_document_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/create_document_core_post/{template_id}")]
// Put and create core of document that we need and redirect to page where
// user can create the document sections with the linkable id.
pub async fn create_document_core_post(
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
        let security_classification = form.security_classification.clone();

        let machine_translate = match form.machine_translate.as_str() {
            "true" => true,
            _ => false,
        };

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // create document
        let document = InsertableDocument::new(
            template_id,
            raw_title,
            raw_purpose,
            &lang,
            user.id,
            security_classification,
            machine_translate,
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

            let _text = Text::create(&default_text, machine_translate)
                .expect("Unable to create text");
        }

        return HttpResponse::Found().header("Location", format!("/{}/edit_document_sections/{}", &lang, &document.id)).finish()
    }
}

#[get("/{lang}/edit_document_core_form/{document_id}/{document_view}")]
/// Form to edit an existing document_core
pub async fn edit_document_core_form(
    data: web::Data<AppData>,
    web::Path((lang, document_id, document_view)): web::Path<(String, Uuid, String)>,
    id: Identity,
    req:HttpRequest,
) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let document_core = Document::get_readable_core_by_id(
            document_id, &lang, false, false)
            .expect("Unable to load document core");

        ctx.insert("document_core", &document_core);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("document_core/edit_document_core_form.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/edit_document_core_put/{document_id}/{document_view}")]
// Put and update an existing document core
pub async fn edit_document_core_put(
    data: web::Data<AppData>,
    web::Path((lang, document_id, document_view)): web::Path<(String, Uuid, String)>,
    form: web::Form<DocumentForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    // validate authorized to edit document
    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        return err.error_response()
    } else {

        let redact = match document_view.as_str() {
            "internal" => false,
            _ => true,
        };

        let raw_title_text = form.title.trim().to_string();
        let raw_purpose_text = form.purpose.trim().to_string();
        let security_classification = form.security_classification.clone();

        let machine_translate = match form.machine_translate.as_str() {
            "true" => true,
            _ => false,
        };

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // update document_core
        let document_core = Document::get_readable_core_by_id(
            document_id, &lang, false, false)
            .expect("Unable to load document");

        if document_core.title_text != raw_title_text {
            let _title_text = Text::update(
                document_core.title_text_id,
                raw_title_text,
                &lang,
                user.id,
                machine_translate,
            ).expect("Unable to update text");
        };

        if document_core.purpose_text != raw_purpose_text {
            let _purpose_text = Text::update(
                document_core.purpose_text_id,
                raw_purpose_text,
                &lang,
                user.id,
                machine_translate,
            ).expect("Unable to update text");
        };

        let mut document = Document::get_by_id(document_id)
            .expect("Unable to load document");

        if document.security_classification != security_classification {
            document.security_classification = security_classification;
        };

        document.updated_at = chrono::Utc::now().naive_utc();
        
        Document::update(&document)
            .expect("Unable to update document");

        let readable_document_core = Document::get_readable_core_by_id(
            document_id, &lang, true, redact)
            .expect("Unable to get readable document");

        ctx.insert("document_core", &readable_document_core);
        ctx.insert("document_view", "internal");

        let rendered = data.tmpl.render("document_core/document_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}