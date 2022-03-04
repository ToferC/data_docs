use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web, ResponseError};
use actix_identity::{Identity};
use inflector::Inflector;
use serde::{Deserialize};
use uuid::Uuid;

use crate::{AppData, extract_identity_data, generate_basic_context};
use crate::models::{Template, Text};
use crate::errors::CustomError;

#[get("/{lang}/text/{text_id}")]
pub async fn get_text(
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

        let rendered = data.tmpl.render("templates/texts/text.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}