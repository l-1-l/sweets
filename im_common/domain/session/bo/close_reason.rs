use super::SessionCloseStatus;
use crate::access::common::ResponseStatus;

pub struct CloseReason {
    close_status: SessionCloseStatus,
    bussiness_status: Option<ResponseStatus>,
    reason: Option<String>,
}

impl CloseReason {
    pub fn new(
        close_status: SessionCloseStatus,
        bussiness_status: Option<ResponseStatus>,
        reason: Option<String>,
    ) -> Self {
        CloseReason {
            close_status,
            bussiness_status,
            reason,
        }
    }

    #[inline]
    pub fn close_status(&self) -> &SessionCloseStatus {
        &self.close_status
    }

    #[inline]
    pub fn is_server_error(&self) -> bool {
        // self.bussiness_status.is_some_and(|&x| x.is_server_error())

        self.bussiness_status
            .filter(|x| x.is_server_error())
            .is_some()
            || self.close_status.is_server_error()
    }
}

impl From<ResponseStatus> for CloseReason {
    fn from(status: ResponseStatus) -> Self {
        let close_status: SessionCloseStatus;

        if status.is_server_error() {
            close_status = SessionCloseStatus::ServerError;
        } else if status.is_illegal_request() {
            close_status = SessionCloseStatus::IllegalRequest;
        } else if status == ResponseStatus::SERVER_UNAVAILABLE {
            close_status = SessionCloseStatus::ServerUnavailable;
        } else {
            close_status = SessionCloseStatus::UnknownError;
        }

        CloseReason::new(
            close_status,
            Some(status),
            Some(status.reason().to_owned()),
        )
    }
}
