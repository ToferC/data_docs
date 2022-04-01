use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;

use crate::{AppData, generate_basic_context};
use crate::handlers::{TemplateCoreForm, NewTemplateCoreForm};
use crate::models::{Template, User, Text, TemplateSection};
use crate::errors::CustomError;

#[get("/{lang}/template_core/{template_id}")]
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

        ctx.insert("template_core", &template_core);

        let rendered = data.tmpl.render("template_core/template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[get("/{lang}/create_template_core")]
/// Form for creating a template_core
pub async fn create_template_core_form(
    data: web::Data<AppData>,
    web::Path(lang): web::Path<String>,
    id: Identity,
    req:HttpRequest) -> impl Responder {

    let (mut ctx, _session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

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

        let rendered = data.tmpl.render("template_core/create_template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/{lang}/save_template_core/{template_id}")]
// Post and create new template_core and forward to create_template_sections
pub async fn save_template_core(
    _data: web::Data<AppData>,
    web::Path((lang, template_id)): web::Path<(String, Uuid)>,
    form: web::Form<NewTemplateCoreForm>,
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
        let raw_name_text = form.name_text.trim().to_string();
        let raw_purpose_text = form.purpose_text.trim().to_string();

        let user = User::find_from_slug(&session_user).expect("Unable to find user");

        // create template
        let template_core = crate::models::Template::create_with_id(
            template_id,
            raw_name_text,
            raw_purpose_text,
            &lang,
            user.id,
        ).expect("Unable to create template");

        for i in 0..form.number_of_sections {
            let _ts = TemplateSection::create_default(
                template_core.id,
                i,
                &lang,
                user.id,
            ).expect("Unable to create default section");
        };

        return HttpResponse::Found().header("Location", format!("/{}/edit_template_sections/{}", &lang, &template_core.id)).finish()
    }
}

#[get("/{lang}/edit_template_core_form/{template_id}")]
/// Form to edit an existing template_core
pub async fn edit_template_core_form(
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

        ctx.insert("template_core", &template_core);

        let rendered = data.tmpl.render("template_core/edit_template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[put("/{lang}/edit_template_core/{template_id}")]
// Put and update an existing template
pub async fn edit_template_core(
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

        // update template_core
        let template_core = crate::models::Template::get_readable_core_by_id(template_id, &lang)
            .expect("Unable to load template");

        if template_core.name_text != raw_name_text {
            let _name_text = Text::update(
                template_core.name_text_id,
                raw_name_text,
                &lang,
                user.id,
            ).expect("Unable to update text");
        };

        if template_core.purpose_text != raw_purpose_text {
            let _purpose_text = Text::update(
                template_core.purpose_text_id,
                raw_purpose_text,
                &lang,
                user.id,
            ).expect("Unable to update text");
        };

        let readable_template_core = Template::get_readable_core_by_id(template_id, &lang)
            .expect("Unable to get readable template");

        ctx.insert("template_core", &readable_template_core);

        let rendered = data.tmpl.render("template_core/template_core.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)

        /*
        return HttpResponse::Found()
            .header("Content-Type", "GET")
            .header("Location", format!("/{}/template_core/{}", &lang, &template_core.id))
            .finish()
        */
    }
}