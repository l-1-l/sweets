use crate::domain::{
    account::AccountId,
    token::{TokenId, TokenRepositoryExt},
};
use common::{
    async_trait,
    deadpool_redis::{redis::AsyncCommands, Pool},
    Config, Result,
};
use std::sync::Arc;

pub struct AuthTokenInmemRepo(pub Arc<Pool>);

impl AuthTokenInmemRepo {
    fn key(&self, token_id: &TokenId) -> String {
        format!("token_jwt_{}", token_id.to_string(),)
    }
}

#[async_trait]
impl TokenRepositoryExt for AuthTokenInmemRepo {
    async fn get(&self, key: &TokenId) -> Result<Option<AccountId>> {
        let mut conn = self.0.get().await?;

        let id = conn
            .get::<String, Option<Vec<u8>>>(self.key(key))
            .await?
            .map(AccountId::from);

        Ok(id)
    }

    async fn set(&self, key: &TokenId, val: &AccountId) -> Result<()> {
        let mut conn = self.0.get().await?;
        let config = Config::get().secrets();

        conn.set_ex(self.key(key), val.as_ref(), config.access_expire_in())
            .await?;

        Ok(())
    }

    async fn del(&self, key: &TokenId) -> Result<()> {
        let mut conn = self.0.get().await?;

        conn.del(self.key(key)).await?;

        Ok(())
    }
}
