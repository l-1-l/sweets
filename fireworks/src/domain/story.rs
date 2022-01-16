mod aspect_ratio;
mod content;
mod kind;
mod media;
mod repository;
mod video;

pub use aspect_ratio::AspectRatio;
pub use content::StoryContent;
pub use kind::StoryKind;
pub use media::StoryMedia;
pub use repository::StoryRepoExt;

use common::{
    models::{AggregateRoot, Id},
    Result,
};

pub type AuthorId = Id;
pub type StoryId = Id;

pub struct Story {
    base: AggregateRoot<StoryId>,
    author_id: AuthorId,
    content: Option<StoryContent>,
    is_public: bool,
    invisible: bool,
}

impl Story {
    pub fn new(
        id: AuthorId,
        author_id: AuthorId,
        content: Option<StoryContent>,
        is_public: bool,
        invisible: bool,
    ) -> Result<Self> {
        let story = Story {
            base: AggregateRoot::new(id),
            author_id,
            content,
            is_public,
            invisible,
        };

        Ok(story)
    }
    pub fn build(
        base: AggregateRoot<StoryId>,
        author_id: AuthorId,
        content: Option<StoryContent>,
        is_public: bool,
        invisible: bool,
    ) -> Self {
        Story {
            base,
            author_id,
            content,
            is_public,
            invisible,
        }
    }
    pub fn base(&self) -> &AggregateRoot<StoryId> {
        &self.base
    }
    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }
    pub fn content(&self) -> Option<&StoryContent> {
        self.content.as_ref()
    }
    pub fn invisible(&self) -> bool {
        self.invisible
    }
    pub fn is_public(&self) -> bool {
        self.is_public
    }
    // pub fn media(&self) -> Option<&StoryMedia> {
    //     self.media.as_ref()
    // }
}
