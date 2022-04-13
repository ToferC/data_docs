use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::handlers::{TemplateSectionForm};
use crate::models::{TemplateSection, User, Template, Text};
use crate::errors::CustomError;

#[get("/{lang}/get_template_section/{template_section_id}")]
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

        ctx.insert("template_section", &template_section);

        let rendered = data.tmpl.render("template_sections/template_section.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/create_template_sections/{template_section_id}")]
/// Get and render form to create template section
pub async fn create_template_sections(
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

        let template_core = Template::get_readable_core_by_id(template_section_id, &lang)
            .expect("Unable to load template section");

        ctx.insert("template_core", &template_core);

        let rendered = data.tmpl.render("template_sections/create_template_sections.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/edit_template_section/{template_section_id}")]
pub async fn edit_template_section_form(
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
            .expect("Unable to load template section");

        ctx.insert("template_section", &template_section);

        let rendered = data.tmpl.render("template_sections/edit_template_section.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/edit_template_section/{template_section_id}")]
pub async fn edit_template_section(
    data: web::Data<AppData>,
    web::Path((lang, template_section_id)): web::Path<(String, Uuid)>,
    form: web::Form<TemplateSectionForm>,
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

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        let template_section = TemplateSection::get_readable_by_id(template_section_id, &lang)
            .expect("Unable to load template section");

        // validate authorized to edit document
        let raw_header_text = form.header_text.trim().to_string();
        let raw_instructions_text = form.instructions_text.trim().to_string();
        let raw_help_text = form.help_text.trim().to_string();
        let raw_character_limit = form.character_limit;

        if template_section.header_text != raw_header_text {
            let _header_text = Text::update(
                template_section.header_text_id,
                raw_header_text,
                &lang,
                user.id,
                false,
            ).expect("Unable to update text");
        };

        if template_section.instructions_text != raw_instructions_text {
            let _instructions_text = Text::update(
                template_section.instructions_text_id,
                raw_instructions_text,
                &lang,
                user.id,
                false,
            ).expect("Unable to update text");
        };
        
        if template_section.help_text != raw_help_text {
            let _help_text = Text::update(
                template_section.help_text_id,
                raw_help_text,
                &lang,
                user.id,
                false,
            ).expect("Unable to update text");
        };

        let mut ts = TemplateSection::get_by_id(template_section.id)
            .expect("Unable to load template_section");

        let character_limit = match raw_character_limit {
            0 => None,
            _ => Some(raw_character_limit),
        };

        if ts.character_limit != character_limit {
            ts.character_limit = character_limit;
        };

        if ts.order_number != form.order_number {
            ts.order_number = form.order_number;
        };

        let updated_template_section = TemplateSection::update(&ts)
            .expect("Unable to update template_section");

        let readable_template_section = TemplateSection::get_readable_by_id(
            updated_template_section.id,
            &lang,
        ).expect("Unable to get readable template section");

        ctx.insert("template_section", &readable_template_section);

        let rendered = data.tmpl.render("template_sections/template_section.html", &ctx).unwrap();
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

        let rendered = data.tmpl.render("template_sections/template_section.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}