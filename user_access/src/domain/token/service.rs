use crate::domain::{
    account::AccountId,
    token::{encoder::TokenEncoderExt, TokenRepositoryExt},
};
use common::{Error, Result};
use std::sync::Arc;

use super::Token;

// TODO: token 方案更新
pub struct TokenService {
    token_repo: Arc<dyn TokenRepositoryExt>,
    token_encoder: Arc<dyn TokenEncoderExt>,
}

impl TokenService {
    pub fn new(
        token_repo: Arc<dyn TokenRepositoryExt>,
        token_encoder: Arc<dyn TokenEncoderExt>,
    ) -> Self {
        Self {
            token_repo,
            token_encoder,
        }
    }

    pub async fn create(&self, account_id: &AccountId) -> Result<Token> {
        let token_id = self.token_repo.next_id().await?;
        let token = self.token_encoder.encode(&token_id)?;
        self.token_repo.set(&token_id, account_id).await?;

        Ok(token)
    }

    pub async fn validate(&self, token: &Token) -> Result<AccountId> {
        let token_id = self.token_encoder.decode(token)?;

        self.token_repo.get(&token_id).await?.ok_or_else(|| {
            Error::forbidden("token ").set_message("token is expired")
        })
    }

    pub async fn invalidate(&self, token: &Token) -> Result<()> {
        let token_id = self.token_encoder.decode(token)?;
        self.token_repo.del(&token_id).await?;

        Ok(())
    }
}
