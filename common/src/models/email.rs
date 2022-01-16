use std::borrow::Cow;
use std::fmt;

use crate::validator::email::validate;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn parse<'a, T>(val: T) -> Result<Self>
    where
        T: Into<Cow<'a, str>>,
    {
        let email = val.into();
        if !validate(email.as_ref()) {
            return Err(Error::bad_format("email"));
        }

        Ok(Email(email.to_string()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
