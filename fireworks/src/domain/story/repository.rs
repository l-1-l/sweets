use common::{async_trait, Result};

use super::{AuthorId, Story, StoryId};

#[async_trait]
pub trait StoryRepoExt: Sync + Send {
    async fn next_id(&self) -> Result<StoryId> {
        Ok(StoryId::next())
    }
    async fn save(&self, story: &Story) -> Result<()>;
    async fn find_by_id(&self, id: &StoryId) -> Result<Story>;
    async fn list_by_ids(&self, ids: &[StoryId]) -> Result<Vec<Story>>;
    async fn list_by_author_id(
        &self,
        author_id: &AuthorId,
    ) -> Result<Vec<Story>>;
}
