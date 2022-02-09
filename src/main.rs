use actix_web::{HttpServer, App, middleware};
use dotenv::dotenv;
use std::env;
use tera::{Tera};
use tera_text_filters::snake_case;

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
async fn main() {

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

    let _secret_key = env::var("SECRET_KEY").expect("Unable to find secret key");

    HttpServer::new(move || {
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto-reload with Tera");
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));

        App::new()
            .configure(handlers::configure_services)
            .app_data("Default")
            .wrap(middleware::Logger::default())
    })
    .bind((host, port))?
    .run()
    .await
}
