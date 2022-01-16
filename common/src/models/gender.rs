use core::fmt;

use crate::{Error, Result};
#[derive(Debug, Clone)]
pub enum Gender {
    Female,
    Male,
    Unknown,
}

impl Gender {
    pub fn parse(gender: &str) -> Result<Self> {
        match gender {
            "Female" => Ok(Gender::Female),
            "Male" => Ok(Gender::Male),
            "Unknown" => Ok(Gender::Unknown),
            _ => Err(Error::bad_format("gender")),
        }
    }

    pub fn build(gender: &str) -> Self {
        match gender {
            "Femalel" => Gender::Female,
            "Male" => Gender::Male,
            _ => Gender::Unknown,
        }
    }
}

impl AsRef<str> for Gender {
    fn as_ref(&self) -> &str {
        match self {
            Gender::Female => "Female",
            Gender::Male => "Male",
            Gender::Unknown => "Uknown",
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
