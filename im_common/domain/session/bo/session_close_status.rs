use std::fmt::Display;

// static CODE_POOL: Lazy<Mutex<HashMap<u16, SessionCloseStatus>>> =
//     Lazy::new(|| {
//         let mut m = HashMap::new();
//         Mutex::new(m)
//     });

#[derive(Debug, Clone, Copy, Eq)]
pub enum SessionCloseStatus {
    //**********************************************************
    //* Closed due to client misbehavior
    //**********************************************************
    IllegalRequest,
    HeartBeatTimeout,
    LoginTimeout,
    Switch,

    //**********************************************************
    //* Closed due to server behavior
    //**********************************************************
    ServerError,
    ServerClosed,
    ServerUnavailable, //TODO: reserved

    //**********************************************************
    //* Closed due to network error
    //**********************************************************
    ConnectionClosed,

    //**********************************************************
    //* Closed due to unknown error
    //**********************************************************
    UnknownError,

    //**********************************************************
    //* Closed by user actively
    //**********************************************************
    // DISCONNECTED_BY_CLIENT(500),
    DisconnectedByClient,
    DisconnectedByOtherDevice,

    //**********************************************************
    //* Closed by admin actively
    //**********************************************************
    DisconnectedByAdmin,

    //**********************************************************
    //* Closed due to the change of user status
    //**********************************************************
    UserIsDeletedOrInactivated,
    UserIsBlocked,
}

impl SessionCloseStatus {
    pub fn as_u16(&self) -> u16 {
        match self {
            SessionCloseStatus::IllegalRequest => 100,
            SessionCloseStatus::HeartBeatTimeout => 110,
            SessionCloseStatus::LoginTimeout => 111,
            SessionCloseStatus::Switch => 112,

            SessionCloseStatus::ServerError => 200,
            SessionCloseStatus::ServerClosed => 201,
            SessionCloseStatus::ServerUnavailable => 202,

            SessionCloseStatus::ConnectionClosed => 300,
            SessionCloseStatus::UnknownError => 400,

            SessionCloseStatus::DisconnectedByClient => 500,
            SessionCloseStatus::DisconnectedByOtherDevice => 501,

            SessionCloseStatus::DisconnectedByAdmin => 600,

            SessionCloseStatus::UserIsDeletedOrInactivated => 700,
            SessionCloseStatus::UserIsBlocked => 701,
        }
    }

    pub fn from_u16(status: u16) -> Self {
        match status {
            100 => SessionCloseStatus::IllegalRequest,
            110 => SessionCloseStatus::HeartBeatTimeout,
            111 => SessionCloseStatus::LoginTimeout,
            112 => SessionCloseStatus::Switch,

            200 => SessionCloseStatus::ServerError,
            201 => SessionCloseStatus::ServerClosed,
            202 => SessionCloseStatus::ServerUnavailable,

            300 => SessionCloseStatus::ConnectionClosed,
            400 => SessionCloseStatus::UnknownError,

            500 => SessionCloseStatus::DisconnectedByClient,
            501 => SessionCloseStatus::DisconnectedByOtherDevice,

            600 => SessionCloseStatus::DisconnectedByAdmin,

            700 => SessionCloseStatus::UserIsDeletedOrInactivated,
            701 => SessionCloseStatus::UserIsBlocked,
            _ => SessionCloseStatus::UnknownError,
        }
    }

    pub fn is_server_error(&self) -> bool {
        (200..300).contains(&self.as_u16())
    }
}

impl Display for SessionCloseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            SessionCloseStatus::IllegalRequest => "IllegalRequest",
            SessionCloseStatus::HeartBeatTimeout => "HeartBeatTimeout",
            SessionCloseStatus::LoginTimeout => "LoginTimeout",
            SessionCloseStatus::Switch => "Switch",
            SessionCloseStatus::ServerError => "ServerError",
            SessionCloseStatus::ServerClosed => "ServerClosed",
            SessionCloseStatus::ServerUnavailable => "ServerUnavailable",
            SessionCloseStatus::ConnectionClosed => "ConnectionClosed",
            SessionCloseStatus::UnknownError => "UnknownError",
            SessionCloseStatus::DisconnectedByClient => "DisconnectedByClient",
            SessionCloseStatus::DisconnectedByOtherDevice => {
                "DisconnectedByOtherDevice"
            }
            SessionCloseStatus::DisconnectedByAdmin => "DisconnectedByAdmin",
            SessionCloseStatus::UserIsDeletedOrInactivated => {
                "UserIsDeletedOrInactivated"
            }
            SessionCloseStatus::UserIsBlocked => "UserIsBlocked",
        };

        write!(f, "{}", status)
    }
}

impl PartialEq<u16> for SessionCloseStatus {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<SessionCloseStatus> for SessionCloseStatus {
    fn eq(&self, other: &SessionCloseStatus) -> bool {
        self.as_u16() == other.as_u16()
    }
}
