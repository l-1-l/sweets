use crate::domain::story::StoryMedia;
use std::fmt;

pub enum StoryKind {
    Text,
    Media(StoryMedia),
}

impl StoryKind {
    pub fn build<S: Into<String>>(kind: S) -> StoryKind {
        StoryKind::Text
    }
}

impl fmt::Display for StoryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoryKind::Text => write!(f, "text"),
            StoryKind::Media(media) => write!(f, "{}", media.to_string()),
        }
    }
}
