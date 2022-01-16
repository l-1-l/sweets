use std::fmt;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Bio(pub String);

impl Bio {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let desc = s.into();

        if desc.len() > 160 {
            return Err(Error::bad_format("bio")
                .set_message("bio length must be between 1 and 160"));
        }

        Ok(Bio(desc))
    }
}

impl AsRef<str> for Bio {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Bio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
