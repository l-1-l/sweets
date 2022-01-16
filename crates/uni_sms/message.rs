mod kind;
mod mobile;
mod request;
mod response;

use super::signature::Signature;
use request::UniRequest;
use reqwest::Client;
use response::UniResponse;
use serde::Serialize;

pub use kind::MessageKind;
pub use mobile::UniMobile;
pub use response::{UniMessageError, UniMessageErrorCode};

#[derive(Debug, Clone)]
pub struct UniMessageSms {
    signature: Signature,
    sign_name: String,
    kind: MessageKind,
}

impl UniMessageSms {
    pub fn new(
        signature: Signature,
        sign_name: String,
        kind: MessageKind,
    ) -> Self {
        UniMessageSms {
            signature,
            sign_name,
            kind,
        }
    }
    async fn request<T: Serialize>(
        &self,
        to: UniMobile,
        params: Option<T>,
    ) -> Result<UniResponse, UniMessageError> {
        let payload = UniRequest::new(
            to,
            self.sign_name.clone(),
            self.kind.clone(),
            params,
        );

        let url =
            format!("https://uni.apistd.com?{}", &self.signature.generate());

        // let data: UniResponse
        let response = Client::new()
            .post(url)
            .basic_auth(&self.signature.id, Some(&self.signature.secret))
            .header("Content-Type", "application/json")
            // .query(&self.signature.generate())
            .json(&payload)
            .send()
            .await?;

        // TODO: error handler

        let data = response.json::<UniResponse>().await?;

        Ok(data)
    }

    pub async fn send_with_params<T: Serialize>(
        &self,
        to: UniMobile,
        params: T,
    ) -> Result<UniResponse, UniMessageError> {
        self.request(to, Some(params)).await
    }

    pub async fn send(
        &self,
        to: UniMobile,
    ) -> Result<UniResponse, UniMessageError> {
        self.request::<()>(to, None).await
    }
}
