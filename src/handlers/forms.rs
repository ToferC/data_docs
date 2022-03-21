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
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    user_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyForm {
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct PasswordForm {
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}