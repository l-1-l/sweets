use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]

pub enum RustEnv {
    Dev,
    Prod,
    Staging,
}

impl Default for RustEnv {
    fn default() -> Self {
        RustEnv::Dev
    }
}

impl fmt::Display for RustEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustEnv::Dev => write!(f, "Dev"),
            RustEnv::Prod => write!(f, "Prod"),
            RustEnv::Staging => write!(f, "Staging"),
        }
    }
}

impl FromStr for RustEnv {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dev" => Ok(RustEnv::Dev),
            "prod" => Ok(RustEnv::Prod),
            "staging" => Ok(RustEnv::Staging),
            _ => Err("invalid RUST_ENV"),
        }
    }
}

impl From<String> for RustEnv {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "production" => RustEnv::Prod,
            "staging" => RustEnv::Staging,
            _ => RustEnv::Dev,
        }
    }
}
