use crate::domain::Email;

#[derive(Clone)]
pub struct User {
    pub email: Email,
    pub password: String,
    pub requires_2fa: bool
}

impl User {
    pub fn new(email: Email, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa
        }
    }
}