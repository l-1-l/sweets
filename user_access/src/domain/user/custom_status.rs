use common::chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CustomStatus {
    emoji: Option<String>,
    text: Option<String>,
    expire_at: Option<DateTime<Utc>>,
}
