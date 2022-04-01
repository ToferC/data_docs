use actix_web::{web, get, HttpResponse, HttpRequest, Responder};
use actix_identity::Identity;
use crate::{AppData, generate_basic_context};

use crate::models::{Template};

#[get("/{lang}/api/templates")]
pub async fn template_api(
    _data: web::Data<AppData>,
    params: web::Path<String>,

    id: Identity,
    req: HttpRequest,
) -> impl Responder {

    let lang = params.into_inner();
    let (_ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());


    let data = Template::get_all_with_data(&lang).unwrap();

    HttpResponse::Ok().json(data)
}