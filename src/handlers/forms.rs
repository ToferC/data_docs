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