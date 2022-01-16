use crate::domain::token::{Token, TokenEncoderExt, TokenId};
use common::{
    chrono::{Duration, Utc},
    Config, Error, Result,
};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey<'static>,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret).into_static(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug)]
pub struct JWTEncoder {
    keys: Keys,
    company: String,
    expire_in: usize,
}

impl Default for JWTEncoder {
    fn default() -> Self {
        let secrets = Config::get().secrets();

        Self {
            keys: Keys::new(secrets.jwt().as_bytes()),
            company: secrets.company().to_string(),
            expire_in: secrets.access_expire_in(),
        }
    }
}

impl TokenEncoderExt for JWTEncoder {
    fn encode(&self, token_id: &TokenId) -> Result<Token> {
        let dt = Utc::now() + Duration::seconds(self.expire_in as i64);

        let claims = Claims {
            sub: token_id.to_string(),
            company: self.company.clone(),
            exp: dt.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.keys.encoding)
            .map_err(|err| Error::internal("token", "encode").wrap_raw(err))?;

        Ok(Token::new(token))
    }

    fn decode(&self, token: &Token) -> Result<TokenId> {
        let token_data = match decode::<Claims>(
            token.as_ref(),
            &self.keys.decoding,
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(err) => return Err(Error::forbidden("token").wrap_raw(err)),
        };

        TokenId::parse(&token_data.claims.sub)
            .map_err(|err| Error::internal("token", "decode").wrap_raw(err))
    }
}
