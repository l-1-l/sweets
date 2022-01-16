use std::fmt;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct UniResponseMessage {
    id: String,
    to: String,
    region_code: String,
    country_code: String,
    message_count: u128,
    message_length: u32,
    status: String,
    upstream: String,
    price: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    // status: String,
    recipients: u128,
    message_count: u128,
    total_amount: String,
    pay_amount: String,
    virtual_amount: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniResponse {
    pub code: String,
    pub message: String,
    pub data: Option<Data>,
}

#[derive(Deserialize)]
pub enum UniMessageErrorCode {
    Invalid,
    Internal,
    Upstream,
    Unknown,
    Account,
}

impl fmt::Display for UniMessageErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniMessageErrorCode::Invalid => write!(f, "Invalid"),
            UniMessageErrorCode::Internal => write!(f, "Internal"),
            UniMessageErrorCode::Account => write!(f, "Account"),
            UniMessageErrorCode::Upstream => write!(f, "Upstream"),
            UniMessageErrorCode::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Deserialize)]
pub struct UniMessageError {
    pub code: UniMessageErrorCode,
    pub message: String,
    pub status: u16,
}

impl From<reqwest::Error> for UniMessageError {
    fn from(e: reqwest::Error) -> Self {
        match e.status() {
            Some(status) => {
                if status.is_server_error() {
                    UniMessageError {
                        code: UniMessageErrorCode::Upstream,
                        message: e.to_string(),
                        status: status.as_u16(),
                    }
                } else {
                    UniMessageError {
                        code: UniMessageErrorCode::Invalid,
                        message: e.to_string(),
                        status: status.as_u16(),
                    }
                }
            }
            None => UniMessageError {
                code: UniMessageErrorCode::Internal,
                message: e.to_string(),
                status: 500,
            },
        }
    }
}
