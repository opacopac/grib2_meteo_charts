use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;
use crate::grib2_section0::grib2_section0_reader::Grib2Section0Reader;
use crate::grib2_section1::grib2_section1_reader::Grib2Section1Reader;

pub struct Grib2CloudCoverReader;


impl Grib2CloudCoverReader {
    pub fn read_file(filename: &str) -> Result<Grib2CloudCoverLayer, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let section0 = Grib2Section0Reader::read(&mut reader)?;
        let section1 = Grib2Section1Reader::read(&mut reader)?;
        let layer = Grib2CloudCoverLayer::new(
            section0,
            section1
        );

        return Ok(layer);
    }
}
