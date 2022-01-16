use std::fmt;

use common::{Error, Result};

#[derive(Debug, Clone)]
pub struct UserName(pub String);

impl UserName {
    pub fn parse<S: Into<String>>(name: S) -> Result<UserName> {
        let v = name.into();
        let len = v.len();

        if len == 0 || len > 50 {
            return Err(Error::bad_format("user.name")
                .set_message("name length must be between 1 and 12"));
        }

        Ok(UserName(v))
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
