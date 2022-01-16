use std::collections::HashMap;

use axum::{body::BoxBody, response::IntoResponse, Json};
use common::config::RustEnv;
use hyper::StatusCode;
use serde::Serialize;

use common::Config;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    kind: String,
    path: String,
    code: String,
    status: Option<u16>,
    message: Option<String>,
    context: HashMap<String, String>,
    cause: Option<Box<Error>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

// App Errors
use common::error::{Error as AppError, ErrorKind};
impl From<AppError> for Error {
    fn from(err: AppError) -> Self {
        let app_env = Config::get().rust_env();
        let is_development = app_env != &RustEnv::Dev;
        if is_development {
            if let ErrorKind::Internal = err.kind() {
                return Error {
                    kind: ErrorKind::Application.to_string(),
                    code: "internal_server".to_owned(),
                    path: "error".to_owned(),
                    status: Some(500),
                    message: None,
                    context: HashMap::new(),
                    cause: None,
                };
            }
        }

        let cause = match err.cause() {
            Some(err) => {
                // let is_app_kind = match err.kind() {
                //     ErrorKind::Application => true,
                //     _ => false,
                // };
                let is_app_kind = err.kind() == &ErrorKind::Application;

                if is_app_kind || is_development {
                    Some(Box::new(Self::from(err.clone())))
                } else {
                    None
                }
            }
            _ => None,
        };

        Error {
            kind: err.kind().to_string(),
            code: err.code().to_string(),
            path: err.path().to_string(),
            status: err.status(),
            message: err.message().cloned(),
            context: err.context().clone(),
            cause,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a Error,
}

impl IntoResponse for Error {
    fn into_response(self) -> hyper::Response<BoxBody> {
        let status = match self.status {
            Some(status) => {
                if let Ok(status) = StatusCode::from_u16(status as u16) {
                    status
                } else {
                    StatusCode::BAD_REQUEST
                }
            }
            None => StatusCode::BAD_REQUEST,
        };

        let res = &ErrorResponse { error: &self };
        let body = Json(res);

        (status, body).into_response()
    }
}
