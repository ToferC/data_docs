use actix_web::web;

use crate::handlers::{
    // base
    index,
};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
}
