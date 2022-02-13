use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web, ResponseError};
use actix_identity::{Identity};
use inflector::Inflector;
use serde::{Deserialize};

use crate::{AppData, extract_identity_data, generate_basic_context};
use crate::models::{Template, TemplateSection};
use crate::errors::CustomError;

#[get("/{lang}/template_index")]
pub async fn template_index(
    data: web::Data<AppData>,
    web::Path(lang): web::Path<String>,
    
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

        let templates_data = Template::get_all_readable(&lang)
            .expect("Unable to load templates");

        ctx.insert("templates", &templates_data);

        let rendered = data.tmpl.render("templates/template_index.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/template/{slug}")]
pub async fn template(
    data: web::Data<AppData>,
    web::Path((lang, slug)): web::Path<(String, String)>,
    
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

        let (template, sections) = Template::get_by_slug(&slug, &lang)
            .expect("Unable to load templates");

        ctx.insert("template", &template);
        ctx.insert("sections", &sections);

        let rendered = data.tmpl.render("templates/template.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}