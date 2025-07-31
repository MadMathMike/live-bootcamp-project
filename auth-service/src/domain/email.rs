#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Option<Email> {
        if email.is_empty() || !email.contains("@") {
            None
        } else {
            Some(Email(email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}