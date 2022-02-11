use actix_web::web;

use crate::handlers::{
    // base
    index,
    raw_index,
    //about,
    toggle_language,
    toggle_language_index,
    toggle_language_two,
    toggle_language_three,

    // admin
    admin_edit_user,
    admin_edit_user_post,

    // errors
    internal_server_error,
    not_found,

    // password reset
    request_password_reset,
    request_password_reset_post,
    password_email_sent,
    password_reset,
    password_reset_post,

    // login
    login_handler,
    login_form_input,
    logout,
    
    // registration
    register_form_input,
    register_handler,
    registration_error,

    // email validation
    email_verification,
    resend_email_verification,
    verify_code,

    // users
    user_index,
    user_page_handler,
    edit_user,
    edit_user_post,
    delete_user,
    delete_user_handler,

};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(raw_index);
    //config.service(about);
    config.service(toggle_language);
    config.service(toggle_language_index);
    config.service(toggle_language_two);
    config.service(toggle_language_three);

    // admin
    config.service(admin_edit_user);
    config.service(admin_edit_user_post);

    // errors
    config.service(internal_server_error);
    config.service(not_found);

    // forgot password
    config.service(request_password_reset);
    config.service(request_password_reset_post);
    config.service(password_email_sent);
    config.service(password_reset);
    config.service(password_reset_post);
 
    // login and logout
    config.service(login_handler);
    config.service(login_form_input);
    config.service(logout);

    // registration and validation
    config.service(register_handler);
    config.service(register_form_input);
    config.service(registration_error);
    config.service(email_verification);
    config.service(resend_email_verification);
    config.service(verify_code);
     
     // users 
     config.service(user_page_handler);
     config.service(user_index);
     config.service(edit_user);
     config.service(edit_user_post);
     config.service(delete_user);
     config.service(delete_user_handler);
}
