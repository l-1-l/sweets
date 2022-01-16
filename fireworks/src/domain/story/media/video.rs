use crate::domain::story::AspectRatio;
use common::{
    models::{Blurhash, Image},
    Result,
};

pub struct StoryVideo {
    url: String,
    blurhash: Blurhash,
    background: Image,
    aspect_ratio: AspectRatio,
}

impl StoryVideo {
    pub fn parse<T: Into<String>>(
        url: T,
        blurhash: Blurhash,
        background: Image,
        aspect_ratio: AspectRatio,
    ) -> Result<StoryVideo> {
        Ok(StoryVideo {
            url: url.into(),
            blurhash,
            background,
            aspect_ratio,
        })
    }

    pub fn build(
        url: String,
        blurhash: Blurhash,
        background: Image,
        aspect_ratio: AspectRatio,
    ) -> Self {
        Self {
            url,
            blurhash,
            background,
            aspect_ratio,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn blurhash(&self) -> &Blurhash {
        &self.blurhash
    }
    pub fn background(&self) -> &Image {
        &self.background
    }
    pub fn aspect_ratio(&self) -> &AspectRatio {
        &self.aspect_ratio
    }
}
