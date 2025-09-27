use std::ops::RangeInclusive;

pub struct IconCh1ModelConfig {}


impl IconCh1ModelConfig {
    pub fn get_vertical_level_range() -> RangeInclusive<usize> {
        31..=79
    }
}
