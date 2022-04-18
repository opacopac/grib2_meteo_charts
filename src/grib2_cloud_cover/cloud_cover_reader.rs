use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::grib2_cloud_cover::cloud_cover_layer::CloudCoverLayer;
use crate::grib2_section0::section0_reader::Section0Reader;
use crate::grib2_section1::section1_reader::Section1Reader;
use crate::grib2_section2::section2_reader::Section2Reader;
use crate::grib2_section3::section3_reader::Section3Reader;
use crate::grib2_section4::section4_reader::Section4Reader;
use crate::grib2_section5::section5_reader::Section5Reader;
use crate::grib2_section6::section6_reader::Section6Reader;
use crate::grib2_section7::section7_reader::Section7Reader;
use crate::grib2_section8::section8_reader::Section8Reader;

pub struct CloudCoverReader;


impl CloudCoverReader {
    pub fn read_file(filename: &str) -> Result<CloudCoverLayer, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let section0 = Section0Reader::read(&mut reader)?;
        let section1 = Section1Reader::read(&mut reader)?;
        let section2 = Section2Reader::read(&mut reader)?;
        let section3 = Section3Reader::read(&mut reader)?;
        let section4 = Section4Reader::read(&mut reader)?;
        let section5 = Section5Reader::read(&mut reader)?;
        let section6 = Section6Reader::read(&mut reader)?;
        let section7 = Section7Reader::read(&mut reader)?;
        let section8 = Section8Reader::read(&mut reader)?;
        let layer = CloudCoverLayer::new(
            section0,
            section1,
            section2,
            section3,
            section4,
            section5,
            section6,
            section7,
            section8
        );

        return Ok(layer);
    }
}
