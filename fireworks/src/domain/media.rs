use common::models::{AggregateRoot, Id};

pub type MediaId = Id;

pub struct Media {
    base: AggregateRoot<MediaId>,
}
