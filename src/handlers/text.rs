use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::handlers::TextForm;
use crate::models::{Text, LatestText, InsertableText, User};
use crate::errors::CustomError;

#[get("/{lang}/text/{text_id}/{document_view}")]
pub async fn get_text(
    data: web::Data<AppData>,
    web::Path((lang, text_id, document_view)): web::Path<(String, Uuid, String)>,
    
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

        // Determine view of text to render
        let redact = match document_view.as_str() {
            "internal" => false,
            _ => true,
        };

        let text = Text::get_text_by_id(text_id, &lang).expect("Unable to retrieve text");

        let text = LatestText::get_from(text, true, redact);

        ctx.insert("text", &text);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("texts/text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/create_text/{section_id}")]
pub async fn create_new_text(
    data: web::Data<AppData>,
    web::Path((lang, section_id)): web::Path<(String, Uuid)>,
    form: web::Form<TextForm>,
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
        let content = form.content.trim();

        println!("Saving text: {}", content);

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        let insertable_text = InsertableText::new(Some(section_id), &lang, content.to_string(), user.id);

        println!("Saving text: {:?}", &insertable_text.content);

        let text = Text::create(&insertable_text).expect("Unable to create text");

        println!("Saved!");

        ctx.insert("text", &text);
        ctx.insert("document_view", "internal");

        let rendered = data.tmpl.render("texts/text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/edit_text/{text_id}/{document_view}")]
pub async fn edit_text_form(
    data: web::Data<AppData>,
    web::Path((lang, text_id, document_view)): web::Path<(String, Uuid, String)>,
    
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

        let text = LatestText::get_from(text, false, false);

        ctx.insert("text", &text);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("texts/edit_text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/edit_text/{text_id}/{document_view}")]
pub async fn edit_text_put(
    data: web::Data<AppData>,
    web::Path((lang, text_id, document_view)): web::Path<(String, Uuid, String)>,
    form: web::Form<TextForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    // validate authorized to edit document
    if role == "CHANGE TO NOT SIGNED IN".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        println!("{}", &err);
        return err.error_response()
    } else {

        
        let content = form.content.trim();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        println!("Updating text: {:?}", &content);

        let text = Text::update(text_id, content.to_string(), &lang, user.id).expect("Unable to update Text");

        let text = LatestText::get_from(text, true, false);

        println!("Updated!");

        ctx.insert("text", &text);
        ctx.insert("document_view", &document_view);

        let rendered = data.tmpl.render("texts/text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}