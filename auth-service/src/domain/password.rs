#[derive(Clone, PartialEq)]
pub struct Password(String);

impl Password {
    // TODO: return Option<T> instead
    pub fn parse(password: String) -> Result<Password, ()> {
        if password.chars().count() < 8 {
            Err(())
        } else {
            Ok(Password(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}