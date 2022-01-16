use std::fmt;

pub enum AuthAction {
    Signup,
    Signin,
}

impl fmt::Display for AuthAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthAction::Signup => write!(f, "signup"),
            AuthAction::Signin => write!(f, "signin"),
        }
    }
}
