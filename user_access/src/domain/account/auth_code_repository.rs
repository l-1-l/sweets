use crate::types::AuthAction;
use common::{async_trait, Result};

#[async_trait]
pub trait AuthCodeRepoExt: Sync + Send {
    async fn save(
        &self,
        action: &AuthAction,
        phone: &str,
        code: &str,
    ) -> Result<()>;

    async fn counter(&self, action: &AuthAction, phone: &str) -> Result<i16>;

    async fn get(
        &self,
        action: &AuthAction,
        phone: &str,
    ) -> Result<Option<String>>;

    async fn is_freed(&self, action: &AuthAction, phone: &str) -> Result<bool>;
}
