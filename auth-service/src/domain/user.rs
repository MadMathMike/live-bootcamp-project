pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool
}

impl User {
    pub fn new() -> Self {
        Self {
            email: "garbage".to_owned(),
            password: "garbage".to_owned(),
            requires_2fa: false
        }
    }
}