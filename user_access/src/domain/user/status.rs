use common::{Error, Result};
#[derive(Debug, Clone)]
pub enum UserStatus {
    Online,
    Idle,
    Dnd, // Do Not Disturb
    Invisible,
}

impl UserStatus {
    pub fn parse(status: &str) -> Result<UserStatus> {
        match status {
            "Online" => Ok(UserStatus::Online),
            "Idle" => Ok(UserStatus::Idle),
            "Dnd" => Ok(UserStatus::Dnd),
            "Invisible" => Ok(UserStatus::Invisible),
            _ => Err(Error::bad_format("user_status")
                .set_message(format!("Unknown user status: {}", status))),
        }
    }

    pub fn build(status: &str) -> Self {
        match status {
            "Idle" => UserStatus::Idle,
            "Dnd" => UserStatus::Dnd,
            "Invisible" => UserStatus::Invisible,
            _ => UserStatus::Online,
        }
    }
}

impl AsRef<str> for UserStatus {
    fn as_ref(&self) -> &str {
        match self {
            UserStatus::Online => "Online",
            UserStatus::Idle => "Idle",
            UserStatus::Dnd => "Dnd",
            UserStatus::Invisible => "Invisible",
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
