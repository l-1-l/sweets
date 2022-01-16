use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::event::Event;
use crate::Result;

#[async_trait]
pub trait EventRepositoryExt: Sync + Send {
    async fn search(
        &self,
        // after_id: Option<&EventId>,
        topic: Option<&String>,
        code: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Event>>;

    async fn save(&self, event: &Event) -> Result<()>;
}
