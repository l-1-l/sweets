use crate::domain::story::AspectRatio;
use common::{models::Blurhash, Result};

pub struct StoryPhoto {
    url: String,
    blurhash: Blurhash,
    aspect_ratio: AspectRatio,
}

impl StoryPhoto {
    pub fn parse<T: Into<String>>(
        url: T,
        blurhash: Blurhash,
        aspect_ratio: AspectRatio,
    ) -> Result<Self> {
        Ok(StoryPhoto {
            url: url.into(),
            blurhash,
            aspect_ratio,
        })
    }

    pub fn build(
        url: String,
        blurhash: Blurhash,
        aspect_ratio: AspectRatio,
    ) -> Self {
        StoryPhoto {
            url,
            blurhash,
            aspect_ratio,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn blurhash(&self) -> &Blurhash {
        &self.blurhash
    }
    pub fn aspect_ratio(&self) -> &AspectRatio {
        &self.aspect_ratio
    }
}
