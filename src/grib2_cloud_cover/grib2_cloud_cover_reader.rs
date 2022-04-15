use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;

pub struct Grib2CloudCoverReader;


impl Grib2CloudCoverReader {
    pub fn read_file(filename: &str) -> Result<Grib2CloudCoverLayer, Box<dyn Error>> {
        let file = File::open(filename)?;

        return Ok(Grib2CloudCoverLayer {});
    }
}
