use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web, ResponseError};
use actix_identity::{Identity};
use inflector::Inflector;
use serde::{Deserialize};
use uuid::Uuid;

use crate::{AppData, extract_identity_data, generate_basic_context};
use crate::handlers::{TemplateCoreForm, TemplateSectionForm};
use crate::models::{Template, TemplateSection, User};
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
pub async fn template(
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

        let (template, sections) = Template::get_readable_by_id(template_id, &lang)
            .expect("Unable to load templates");

        ctx.insert("template", &template);
        ctx.insert("sections", &sections);

        let rendered = data.tmpl.render("templates/template.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/create_template}")]
pub async fn create_template(
    data: web::Data<AppData>,
    web::Path(lang): web::Path<String>,
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

        let template_id = Uuid::new_v4();

        ctx.insert("template_id", &template_id);

        let rendered = data.tmpl.render("documents/create_document_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/save_template_core/{template_id}")]
// Post and create template
pub async fn save_template_core(
    data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    form: web::Form<TemplateCoreForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        return err.error_response()
    } else {

        // validate authorized to edit document
        let raw_name_text = form.name_text.trim().to_string();
        let raw_purpose_text = form.purpose_text.trim().to_string();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // create document
        let docs_template = crate::models::Template::create_with_id(
            template_id,
            raw_name_text,
            raw_purpose_text,
            lang,
            user.id,
        ).expect("Unable to create template");

        ctx.insert("template", &docs_template);

        let rendered = data.tmpl.render("templates/template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/save_template_section/{template_id}")]
// Post and create template
pub async fn save_template_section(
    data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    form: web::Form<TemplateSectionForm>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, session_user, role, lang) = generate_basic_context(id, &lang, req.uri().path());

    if role != "user".to_string() &&
        role != "admin".to_string() {
        let err = CustomError::new(
            406,
            "Not authorized".to_string(),
        );
        return err.error_response()
    } else {

        // validate authorized to edit document
        let header_text = form.header_text.trim().to_string();
        let instructions_text = form.instructions_text.trim().to_string();
        let help_text = form.help_text.trim().to_string();
        let raw_character_limit = form.character_limit;

        let character_limit = match raw_character_limit {
            0 => None,
            _ => Some(raw_character_limit),
        };

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // create template section
        let insertable_template_section = crate::models::InsertableTemplateSection::new(
            template_id,
            header_text,
            form.order_number,
            instructions_text,
            help_text,
            character_limit,
            lang,
            user.id,
        ).expect("Unable to create template section");

        let template_section = TemplateSection::create(&insertable_template_section)
            .expect("Unable to save template section");

        ctx.insert("template_section", &template_section);

        let rendered = data.tmpl.render("templates/template_section.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/get_template_core/{template_id}")]
pub async fn get_template_core(
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

        let template_core = Template::get_readable_core_by_id(template_id, &lang)
            .expect("Unable to load template core");

        ctx.insert("template", &template_core);

        let rendered = data.tmpl.render("templates/template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/get_template_core/{template_section_id}")]
pub async fn get_template_section(
    data: web::Data<AppData>,
    web::Path((lang, template_section_id)): web::Path<(String, Uuid)>,
    
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

        let template_section = TemplateSection::get_readable_by_id(template_section_id, &lang)
            .expect("Unable to load template core");

        ctx.insert("template", &template_section);

        let rendered = data.tmpl.render("templates/template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}