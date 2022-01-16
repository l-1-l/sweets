use serde::Serialize;

// pub enum UniMobile<'a, T: AsRef<str>, I: IntoIterator<Item: T>> {
#[derive(Serialize)]
#[serde(untagged)]
pub enum UniMobile {
    Single(String),
    Set(Vec<String>),
}
