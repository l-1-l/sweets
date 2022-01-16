use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Eq)]
pub enum AuthAction {
    Signup,
    Signin
}

impl fmt::Display for AuthAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthAction::Signup => write!(f, "signup"),
            AuthAction::Signin => write!(f, "signin"),
        }
    }
}

impl PartialEq for AuthAction {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (AuthAction::Signup, AuthAction::Signup)
                | (AuthAction::Signin, AuthAction::Signin)
        )
    }
}
