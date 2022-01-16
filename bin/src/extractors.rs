use crate::errors::Error;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BearerToken(pub String);
impl AsRef<str> for BearerToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for BearerToken
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| common::Error::unauthorized())?;

        // let Extension(pool) = Extension::<PgPool>::from_request(req)
        //     .await
        //     .map_err(|err| Error::from(err))?;
        // let claims = jwt::verify(bearer.token())?;
        Ok(BearerToken(bearer.token().into()))
    }
}
