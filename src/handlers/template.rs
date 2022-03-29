use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;
use std::collections::BTreeMap;

use crate::{AppData, generate_basic_context};
use crate::models::{Template};
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

#[get("/{lang}/template/{template_id}")]
pub async fn get_template_page(
    data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    
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

        let (template_core, template_sections) = Template::get_readable_by_id(template_id, &lang)
            .expect("Unable to load templates");

        let mut ordered_template_sections = BTreeMap::new();

        for (_k, v) in template_sections {
            ordered_template_sections.insert(v.order_number, v);
        }

        ctx.insert("template_core", &template_core);
        ctx.insert("template_sections", &ordered_template_sections);

        let rendered = data.tmpl.render("templates/template.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/edit_template_sections/{template_id}")]
pub async fn edit_template_page(
    data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    
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

        let (template_core, template_sections) = Template::get_readable_by_id(template_id, &lang)
            .expect("Unable to load templates");

        let mut ordered_template_sections = BTreeMap::new();

        for (_k, v) in template_sections {
            ordered_template_sections.insert(v.order_number, v);
        }

        ctx.insert("template_core", &template_core);
        ctx.insert("template_sections", &ordered_template_sections);

        let rendered = data.tmpl.render("templates/edit_template_sections.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}