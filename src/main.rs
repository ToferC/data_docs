use dotenv::dotenv;
use std::env;

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello World!"
}

#[get("/")]
fn index() -> &'static str {
    "index"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {

    dotenv().ok();

    let _secret_key = env::var("SECRET_KEY").expect("Unable to find secret key");

    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/hello", routes![world])
}
