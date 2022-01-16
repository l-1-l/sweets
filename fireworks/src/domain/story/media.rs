mod photo;
mod sound;
mod video;

use std::fmt;

pub use photo::*;
pub use sound::*;
pub use video::*;

pub enum StoryMedia {
    Video(StoryVideo),
    Photo(StoryPhoto),
    Sound(StorySound),
}

impl fmt::Display for StoryMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoryMedia::Video(_) => write!(f, "video"),
            StoryMedia::Photo(_) => write!(f, "photo"),
            StoryMedia::Sound(_) => write!(f, "sound"),
        }
    }
}
