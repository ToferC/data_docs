use serde::{Deserialize};

// Business Forms
#[derive(Deserialize, Debug)]
pub struct DocumentForm {
    pub title: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct TextForm {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct TemplateCoreForm {
    pub name_text: String,
    pub purpose_text: String,
}

#[derive(Debug, Deserialize)]
pub struct NewTemplateCoreForm {
    pub name_text: String,
    pub purpose_text: String,
    pub number_of_sections: i32,
}

#[derive(Debug, Deserialize)]
pub struct TemplateSectionForm {
    pub header_text: String,
    pub order_number: i32,
    pub instructions_text: String,
    pub help_text: String,
    pub character_limit: i32,
}


// Administrative Forms

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyForm {
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct PasswordForm {
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}