use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::handlers::MetaDataForm;
use crate::models::{MetaData, LatestMetaData, InsertableMetaData, User};
use crate::errors::CustomError;

#[get("/{lang}/metadata/{metadata_id}/{document_view}")]
pub async fn get_metadata(
    data: web::Data<AppData>,
    web::Path((lang, metadata_id, document_view)): web::Path<(String, Uuid, String)>,
    
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

        // Determine view of metadata to render
        let redact = match document_view.as_str() {
            "internal" => false,
            _ => true,
        };

        let metadata = MetaData::get_metadata_by_id(metadata_id, &lang).expect("Unable to retrieve metadata");

        let metadata = ReadableMetaData::from(metadata, lang);

        ctx.insert("metadata", &metadata);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("metadata/metadata.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/create_metadata/{section_id}")]
pub async fn create_new_metadata(
    data: web::Data<AppData>,
    web::Path((lang, section_id)): web::Path<(String, Uuid)>,
    form: web::Form<MetaDataForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        // validate authorized to edit document
        let content = form.content.trim();

        let machine_translate = match form.machine_translate.as_str() {
            "true" => true,
            _ => false,
        };

        println!("Saving metadata: {}", content);

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        let insertable_metadata = InsertableMetaData::new(Some(section_id), &lang, content.to_string(), user.id);

        println!("Saving metadata: {:?}", &insertable_metadata.content);

        let metadata = MetaData::create(&insertable_metadata, machine_translate).expect("Unable to create metadata");

        println!("Saved!");

        ctx.insert("metadata", &metadata);

        let rendered = data.tmpl.render("metadata/metadata.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/edit_metadata/{metadata_id}/{document_view}")]
pub async fn edit_metadata_form(
    data: web::Data<AppData>,
    web::Path((lang, metadata_id, document_view)): web::Path<(String, Uuid, String)>,
    
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

        let metadata = MetaData::get_metadata_by_id(metadata_id, &lang).expect("Unable to retrieve metadata");

        let metadata = LatestMetaData::get_from(metadata, false, false);

        ctx.insert("metadata", &metadata);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("metadata/edit_metadata.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/edit_metadata/{metadata_id}/{document_view}")]
pub async fn edit_metadata_put(
    data: web::Data<AppData>,
    web::Path((lang, metadata_id, document_view)): web::Path<(String, Uuid, String)>,
    form: web::Form<MetaDataForm>,
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
        println!("{}", &err);
        return err.error_response()
    } else {

        
        let content = form.content.trim();

        let machine_translate = match form.machine_translate.as_str() {
            "true" => true,
            _ => false,
        };

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        println!("Updating metadata: {:?}", &content);

        let metadata = MetaData::update(
            metadata_id, 
            content.to_string(), 
            &lang, user.id, 
            machine_translate)
            .expect("Unable to update MetaData");

        // Determine view of metadata to render
        let redact = match document_view.as_str() {
            "internal" => false,
            _ => true,
        };

        let metadata = LatestMetaData::get_from(metadata, true, redact);

        println!("Updated!");

        ctx.insert("metadata", &metadata);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("metadata/metadata.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}