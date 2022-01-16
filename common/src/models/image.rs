use crate::Result;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Image(pub String);

impl Image {
    pub fn parse<S: Into<String>>(image: S) -> Result<Image> {
        let img = image.into();

        Ok(Self(img))
    }
}

impl AsRef<str> for Image {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
pub struct ImageList(Vec<Image>);

impl ImageList {
    pub fn parse<S: Into<String>>(str: S) -> Result<Self> {
        let str: String = str.into();

        let list = str
            .split(',')
            .map(Image::parse)
            .collect::<Result<Vec<Image>>>()?;

        Ok(Self(list))
    }
}

impl AsRef<Vec<Image>> for ImageList {
    fn as_ref(&self) -> &Vec<Image> {
        &self.0
    }
}

impl Display for ImageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self
            .0
            .iter()
            .map(|x| x.as_ref().to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "{}", list)
    }
}

impl PartialEq for ImageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
