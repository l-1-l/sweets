use std::fmt::Display;

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
    pub fn as_i16(&self) -> i16 {
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

    pub fn from_i16(status: i16) -> Self {
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
