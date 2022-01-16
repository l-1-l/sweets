use common::Result;

pub struct StoryContent(pub String);

impl StoryContent {
    pub fn parse<T: Into<String>>(content: T) -> Result<Self> {
        Ok(Self(content.into()))
    }
}

impl AsRef<str> for StoryContent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
