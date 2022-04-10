use actix_web::{web, get, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};

#[get("/")]
pub async fn raw_index() -> impl Responder {
    return HttpResponse::Found().header("Location", "/en").finish()
}

#[get("/{lang}")]
pub async fn index(
    data: web::Data<AppData>,
    params: web::Path<String>,

    id: Identity,
    req: HttpRequest,
) -> impl Responder {

    let lang = params.into_inner();
    let (mut ctx, _, role, _) = generate_basic_context(id, &lang, req.uri().path());
    
    let mut document_view: String = "open".to_string();
    
    if role == "user".to_string() ||
        role == "admin".to_string() {
            // send to external view
           document_view = "internal".to_string();
    };

    ctx.insert("document_view", &document_view);

    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}