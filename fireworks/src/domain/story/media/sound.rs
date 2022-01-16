use crate::domain::story::AspectRatio;
use common::models::{Blurhash, Image};

pub struct StorySound {
    url: String,
    blurhash: Option<Blurhash>,
    backgroud: Option<Image>,
    aspect_ratio: Option<AspectRatio>,
}

impl StorySound {
    pub fn parse<T: Into<String>>(
        url: T,
        blurhash: Option<Blurhash>,
        backgroud: Option<Image>,
        aspect_ratio: Option<AspectRatio>,
    ) -> StorySound {
        StorySound {
            url: url.into(),
            blurhash,
            backgroud,
            aspect_ratio,
        }
    }
    pub fn build(
        url: String,
        blurhash: Option<Blurhash>,
        backgroud: Option<Image>,
        aspect_ratio: Option<AspectRatio>,
    ) -> Self {
        Self {
            url,
            blurhash,
            backgroud,
            aspect_ratio,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn blurhash(&self) -> Option<&Blurhash> {
        self.blurhash.as_ref()
    }
    pub fn backgroud(&self) -> Option<&Image> {
        self.backgroud.as_ref()
    }
    pub fn aspect_ratio(&self) -> Option<&AspectRatio> {
        self.aspect_ratio.as_ref()
    }
}
