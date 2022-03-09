use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use inflector::Inflector;
use serde::{Deserialize};
use uuid::Uuid;

use crate::{AppData, extract_identity_data, generate_basic_context};
use crate::models::{Text, InsertableText, User, Document, InsertableDocument};
use super::DocumentForm;
use crate::errors::CustomError;

#[get("/{lang}/document/{document_id}")]
pub async fn get_document(
    data: web::Data<AppData>,
    web::Path((lang, document_id)): web::Path<(String, Uuid)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let (document, sections) = Document::get_readable_by_id(document_id, &lang).expect("Unable to retrieve text");

        ctx.insert("document", &document);
        ctx.insert("sections", &document);

        let rendered = data.tmpl.render("templates/documents/document.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/create_document_core")]
pub async fn create_document(
    data: web::Data<AppData>,
    web::Path(lang): web::Path<(String)>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let rendered = data.tmpl.render("templates/documents/create_document_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/save_document_core/")]
// Put and create core of document that we need and redirect to page where
// user can create the document sections with the linkable id.
pub async fn save_document(
    data: web::Data<AppData>,
    web::Path((lang, section_id)): web::Path<(String, Uuid)>,
    form: web::Form<DocumentForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        // validate authorized to edit document
        let raw_title = form.title.trim().to_string();
        let raw_purpose = form.purpose.trim().to_string();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        let document = InsertableDocument::new(
            raw_title,
            raw_purpose,
            &lang,
            user.id,
        ).expect("Unable to generate insertable_document");

        let document = Document::create(&document)
            .expect("Unable to create document");

        return HttpResponse::Found().header("Location", format!("/{}/create_document_sections/{}", &lang, &document.id)).finish()
    }
}

#[get("/{lang}/edit_text/{text_id}")]
pub async fn edit_text(
    data: web::Data<AppData>,
    web::Path((lang, text_id)): web::Path<(String, Uuid)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        let text = Text::get_text_by_id(text_id, &lang).expect("Unable to retrieve text");

        ctx.insert("text", &text);

        let rendered = data.tmpl.render("templates/texts/edit_text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/text/{text_id}")]
pub async fn post_text(
    data: web::Data<AppData>,
    web::Path((lang, text_id)): web::Path<(String, Uuid)>,
    form_text: web::Form<String>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        // validate authorized to edit document
        let content = form_text.trim();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        let text = Text::update(text_id, content.to_string(), &lang, user.id).expect("Unable to update Text");

        ctx.insert("text", &text);

        let rendered = data.tmpl.render("templates/texts/text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}