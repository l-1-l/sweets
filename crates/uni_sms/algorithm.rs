use std::fmt;

#[derive(Debug, Clone)]
pub enum Algorithm {
    HmacSha256,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::HmacSha256 => write!(f, "hmac-sha256"),
        }
    }
}
