use std::fmt::Display;

pub struct Blurhash(pub String);

impl AsRef<str> for Blurhash {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Blurhash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Blurhash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
