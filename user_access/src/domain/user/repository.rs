use common::{async_trait, Result};

use super::User;
use crate::UserId;

#[async_trait]
pub trait UserRepoExt: Sync + Send {
    async fn next_id(&self) -> Result<UserId> {
        Ok(UserId::next())
    }
    async fn find_by_id(&self, id: &UserId) -> Result<User>;
    async fn list_by_ids(&self, ids: &[UserId]) -> Result<Vec<User>>;
    async fn save(&self, user: &User) -> Result<()>;
}
