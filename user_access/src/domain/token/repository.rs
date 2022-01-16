use crate::domain::{account::AccountId, token::TokenId};
use common::{async_trait, Result};

#[async_trait]
pub trait TokenRepositoryExt: Sync + Send {
    async fn next_id(&self) -> Result<TokenId> {
        Ok(TokenId::next())
    }
    async fn get(&self, key: &TokenId) -> Result<Option<AccountId>>;
    async fn set(&self, key: &TokenId, val: &AccountId) -> Result<()>;
    async fn del(&self, key: &TokenId) -> Result<()>;
}
