use std::collections::BTreeMap;

use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::models::{Document, MetaData, ReadableMetaData};

use crate::errors::CustomError;

#[get("/{lang}/document_index/{document_view}")]
pub async fn document_index(
    data: web::Data<AppData>,
    web::Path((lang, document_view)): web::Path<(String, String)>,
    
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

    let (document, sections) = Document::get_all_readable_by_id(
        document_id, &lang, true, redact).expect("Unable to retrieve text");

    let mut ordered_sections = BTreeMap::new();

    for (_k, v) in sections {
        ordered_sections.insert(v.order_number, v);
    };

    let metadata = MetaData::get_by_document_id(document_id).expect("Unable to retrieve metadata for document");

    let readable_metadata = ReadableMetaData::from_metadata(metadata, &lang);

    ctx.insert("document", &document);
    ctx.insert("sections", &ordered_sections);
    ctx.insert("document_view", &document_view);
    ctx.insert("metadata", &readable_metadata);

    let rendered = data.tmpl.render("documents/document.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
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

        let (document, sections) = Document::get_all_readable_by_id(document_id, &lang, false, false)
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

#[get("/{lang}/toggle_document_published/{document_id}/{document_view}")]
pub async fn toggle_document_published(
    _data: web::Data<AppData>,
    web::Path((lang, document_id, document_view)): web::Path<(String, Uuid, String)>,
    
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (_ctx, _session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
            // send to external view
            return HttpResponse::Found().header("Location", format!("/{}/document/{}/open", lang, document_id)).finish()
    };

    let mut document = Document::get_by_id(document_id).expect("Unable to retrieve text");

    document.published = match document.published {
        false => true,
        true => false,
    };

    let d = document.update()
        .expect("Unable to update document");

    HttpResponse::Found().header(
        "Location",
        format!("/{}/document/{}/{}", lang, d.id, &document_view)).finish()
}