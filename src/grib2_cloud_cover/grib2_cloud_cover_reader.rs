use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;
use crate::grib2_cloud_cover::grib2_section0_reader::Grib2Section0Reader;

pub struct Grib2CloudCoverReader;


impl Grib2CloudCoverReader {
    pub fn read_file(filename: &str) -> Result<Grib2CloudCoverLayer, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        Grib2Section0Reader::read(reader)?;

        return Ok(Grib2CloudCoverLayer {});
    }
}
