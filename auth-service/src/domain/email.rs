#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    // TODO: return Option<T> instead
    pub fn parse(email: String) -> Result<Email, ()> {
        if email.is_empty() || !email.contains("@") {
            Err(())
        } else {
            Ok(Email(email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}