mod encoder;
mod repository;
mod service;

pub use encoder::TokenEncoderExt;
pub use repository::TokenRepositoryExt;
use serde::{Deserialize, Serialize};
pub use service::TokenService;

use common::models::Id;

pub type TokenId = Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    pub fn new<T>(token: T) -> Self
    where
        T: std::fmt::Display,
    {
        Token(token.to_string())
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
