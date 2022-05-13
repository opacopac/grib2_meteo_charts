use derive_new::new;

use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

#[derive(Debug, new)]
pub struct GridDefinitionTemplate3_101 {
    pub shape_of_earth: ShapeOfEarth,
    pub number_of_grid: u32,
    pub number_of_grid_in_ref: u8,
    pub hor_grid_uuid: u128
}
