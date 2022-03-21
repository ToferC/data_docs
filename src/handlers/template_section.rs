use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::handlers::{TemplateSectionForm};
use crate::models::{TemplateSection, User};
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

        ctx.insert("template", &template_section);

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