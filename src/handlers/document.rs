use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::models::{Text, User, Template, Document, InsertableDocument};
use super::DocumentForm;
use crate::errors::CustomError;

#[get("/{lang}/document/{document_id}")]
pub async fn get_document(
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
        return err.error_response()
    } else {

        let (document, sections) = Document::get_readable_by_id(document_id, &lang).expect("Unable to retrieve text");

        ctx.insert("document", &document);
        ctx.insert("sections", &sections);

        let rendered = data.tmpl.render("documents/document.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
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

        ctx.insert("template", &template);
        ctx.insert("sections", &sections);

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

        let document = InsertableDocument::new(
            template_id,
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

#[get("/{lang}/create_document_sections/{document_id}")]
pub async fn create_document_sections(
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

        let (document, sections) = Document::get_readable_by_id(document_id, &lang)
            .expect("Unable to load document");

        ctx.insert("document", &document);
        ctx.insert("sections", &sections);

        let rendered = data.tmpl.render("documents/create_document_sections.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}