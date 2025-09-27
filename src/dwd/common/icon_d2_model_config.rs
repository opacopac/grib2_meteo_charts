use std::ops::RangeInclusive;

pub struct IconD2ModelConfig {}


impl IconD2ModelConfig {
    pub fn get_vertical_level_range() -> RangeInclusive<u8> {
        25..=65
    }
}
