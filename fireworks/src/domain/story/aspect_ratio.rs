use common::{Error, Result};

pub struct AspectRatio {
    x: i8,
    y: i8,
}

impl AspectRatio {
    pub fn parse(x: i8, y: i8) -> Result<Self> {
        match (x, y) {
            (4, 3) | (16, 9) | (1, 1) => Ok(AspectRatio { x, y }),
            _ => Err(Error::invalid("aspect_ratio")
                .set_message("invalid aspect ratio")),
        }
    }
    pub fn build(x: i8, y: i8) -> Self {
        AspectRatio { x, y }
    }

    pub fn x(&self) -> i8 {
        self.x
    }

    pub fn y(&self) -> i8 {
        self.y
    }
}

impl PartialEq for AspectRatio {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
