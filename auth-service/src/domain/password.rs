#[derive(Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Option<Password> {
        if password.chars().count() < 8 {
            None
        } else {
            Some(Password(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}