use actix_web::{HttpServer, App, middleware};
use dotenv::dotenv;
use std::env;
use tera::{Tera};
use tera_text_filters::snake_case;
use actix_identity::{IdentityService, CookieIdentityPolicy};

use data_docs::handlers;
use data_docs::AppData;
use data_docs::database;

use fluent_templates::{FluentLoader, static_loader};
// https://lib.rs/crates/fluent-templates

//include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static_loader! {
    static LOCALES = {
        locales: "./i18n/",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8080"))
    };

    let cookie_secret_key = env::var("COOKIE_SECRET_KEY").expect("Unable to find secret key");

    database::init();

    HttpServer::new(move || {
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto-reload with Tera");
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));

        let data = AppData {
            tmpl: tera,
        };

        App::new()
            .configure(handlers::configure_services)
            .data(data.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_secret_key.as_bytes())
                .name("user-auth")
                .secure(false)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
