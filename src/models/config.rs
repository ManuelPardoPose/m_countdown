#[derive(Debug)]
pub struct RgbCol(pub u8, pub u8, pub u8);

pub struct Config {
    col1: RgbCol,
    col2: RgbCol,
    col3: RgbCol,
}

pub struct InvalidColorName {}

impl Config {
    pub fn new(col1: RgbCol, col2: RgbCol, col3: RgbCol) -> Self {
        Self { col1, col2, col3 }
    }

    pub fn get_colors(&self) -> (&RgbCol, &RgbCol, &RgbCol) {
        (&self.col1, &self.col2, &self.col3)
    }
}
