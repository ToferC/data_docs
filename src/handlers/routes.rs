use actix_web::web;

use crate::handlers::{
    // base
    index,
    raw_index,

    // API
    template_api,

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

    // template pages
    template_index,
    get_template_page,

    // template cores
    get_template_core,
    create_template_core_form,
    edit_template_core_form,
    save_template_core,
    edit_template_core,

    // template_sections
    get_template_section,
    edit_template_section_form,
    edit_template_section,
    save_template_section,

    // documents
    document_index,
    get_document,
    create_document_core,
    save_document_core,
    edit_document_sections,

    // text
    get_text,
    create_new_text,
    edit_text_form,
    edit_text_put,


};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(raw_index);
    config.service(template_api);
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

     // template page
     config.service(template_index);
     config.service(get_template_page);
     
     // template_core
     config.service(get_template_core);
     config.service(create_template_core_form);
     config.service(save_template_core);
     config.service(edit_template_core);
     config.service(edit_template_core_form);

     // template_sections
     config.service(get_template_section);
     config.service(save_template_section);
     config.service(edit_template_section_form);
     config.service(edit_template_section);


     // documents
     config.service(document_index);
     config.service(get_document);
     config.service(create_document_core);
     config.service(edit_document_sections);
     config.service(save_document_core);

     // text
     config.service(get_text);
     config.service(create_new_text);
     config.service(edit_text_form);
     config.service(edit_text_put);


}
