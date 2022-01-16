use std::fmt;

use crate::Result;

#[derive(Debug, Clone)]
pub struct Avatar(pub String);

impl Avatar {
    pub fn parse(s: String) -> Result<Self> {
        // TODO: Avatar Rule
        Ok(Avatar(s))
    }
}

impl AsRef<str> for Avatar {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
