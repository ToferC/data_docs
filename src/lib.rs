pub mod models;
pub mod handlers;
pub mod database;
pub mod errors;

use tera::{Tera, Context};
use actix_identity::Identity;
use actix_session::Session;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
embed_migrations!();

#[derive(Clone, Debug)]
pub struct AppData {
    pub tmpl: Tera,
}

/// Generate context, session_user, role and node_names from id and lang
pub fn generate_basic_context(
    id: Identity,
    lang: &str,
    path: &str,
) -> (Context, String, String, String) 
{    
    let mut ctx = Context::new();

    // Get session data and add to context
    let (session_user, role) = extract_identity_data(&id);
    ctx.insert("session_user", &session_user);
    ctx.insert("role", &role);

    let validated_lang = match lang {
        "fr" => "fr",
        "en" => "en",
        _ => "en",
    };

    ctx.insert("lang", &validated_lang);
    ctx.insert("path", &path);

    (ctx, session_user, role, lang.to_owned())
}

pub fn extract_session_data(session: &Session) -> (String, String) {

    let role_data = session.get::<String>("role").expect("Unable to get role from cookie");

    let role = match role_data {
        Some(r) => r,
        None => "".to_string(),
    };

    let user_data = session.get::<String>("user_name").expect("Unable to get user_name from cookie");

    let session_user = match user_data {
        Some(u) => u,
        None => "".to_string(),
    };

    println!("{}-{}", &session_user, &role);

    (session_user, role)
}

pub fn extract_identity_data(id: &Identity) -> (String, String) {

    let id_data = id.identity();

    let session_user = match id_data {
        Some(u) => u,
        None => "".to_string(),
    };

    let user = models::User::find_slim_from_slug(&session_user);

    let role = match user {
        Ok(u) => u.role,
        _ => "".to_string()
    };

    println!("{}-{}", &session_user, &role);

    (session_user, role)
}
