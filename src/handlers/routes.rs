use actix_web::web;

use crate::handlers::{
    // base
    raw_index,
    index,
};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(raw_index);
    config.service(index);
}
