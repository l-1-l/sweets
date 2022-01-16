use std::sync::Arc;

use common::{
    async_trait,
    chrono::{DateTime, Utc},
    models::AggregateRoot,
    pg::{map_rows, Pool, Row},
    Error, Result,
};

use crate::domain::story::{
    AuthorId, Story, StoryContent, StoryId, StoryRepoExt,
};

impl TryFrom<Row> for Story {
    type Error = Error;
    fn try_from(r: Row) -> Result<Self> {
        let id: Vec<u8> = r.try_get("id")?;
        let author_id: Vec<u8> = r.try_get("author_id")?;
        let content: Option<String> = r.try_get("content")?;
        let is_public: bool = r.try_get("is_public")?;
        let invisible: bool = r.try_get("invisible")?;
        // let url: Option<String> = r.try_get("url")?;
        // let blurhash: Option<String> = r.try_get("blurhash")?;
        // let background: Option<String> = r.try_get("background")?;
        // let ratio_x: i8 = r.try_get("ratio_x")?;
        // let ratio_y: i8 = r.try_get("ratio_y")?;
        let created_at: DateTime<Utc> = r.try_get("created_at")?;
        let updated_at: Option<DateTime<Utc>> = r.try_get("updated_at")?;

        Ok(Story::build(
            AggregateRoot::build(StoryId::from(id), created_at, updated_at),
            StoryId::from(author_id),
            content.map(StoryContent),
            is_public,
            invisible,
        ))
    }
}

pub struct StoryRepo(pub Arc<Pool>);

#[async_trait]
impl StoryRepoExt for StoryRepo {
    async fn save(&self, story: &Story) -> Result<()> {
        let client = self.0.get().await?;
        let base = story.base();
        let stmt = "
        INSERT INTO stories (
            id,
            author_id,
            content,
            is_public,
            invisible,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id) DO UPDATE SET
            author_id = $2,
            content = $3,
            is_public = $4,
            invisible = $5,
            updated_at = $7
        ";

        client
            .execute(
                stmt,
                &[
                    &base.id().as_ref(),
                    &story.author_id().as_ref(),
                    &story.content().map(|c| c.as_ref()),
                    &story.is_public(),
                    &story.invisible(),
                    &base.created_at(),
                    &base.updated_at(),
                ],
            )
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, story_id: &StoryId) -> Result<Story> {
        let client = self.0.get().await?;
        let row =client.query_one("SELECT id, author_id, content, is_public, invisible, created_at, updated_at FROM stories where id = $1", &[
            &story_id.as_ref(),
        ]).await
        .map_err(|e| Error::not_found("story").wrap_raw(e))?;

        Ok(Story::try_from(row)?)
    }

    async fn list_by_ids(&self, ids: &[StoryId]) -> Result<Vec<Story>> {
        let client = self.0.get().await?;
        let rows = client.query(
            "SELECT id, author_id, content, is_public, invisible, created_at, updated_at FROM stories where id = ANY($1)",
            &[&ids.iter().map(|id| id.as_ref()).collect::<Vec<&[u8]>>()],
        ).await?;

        Ok(map_rows(rows, Story::try_from)?)
    }

    async fn list_by_author_id(
        &self,
        author_id: &AuthorId,
    ) -> Result<Vec<Story>> {
        let client = self.0.get().await?;
        let rows = client.query(
            "SELECT id, author_id, content, is_public, invisible, created_at, updated_at FROM stories where author_id = $1",
            &[&author_id.as_ref()],
        ).await?;

        Ok(map_rows(rows, Story::try_from)?)
    }
}
