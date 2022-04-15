use std::error::Error;

use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;

pub struct Grib2CloudCoverReader;


impl Grib2CloudCoverReader {
    pub fn read_file(filename: &str) -> Result<Grib2CloudCoverLayer, Box<dyn Error>> {
        return Ok(Grib2CloudCoverLayer {});
    }
}
