use byteorder::{BigEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::grib2_section3::grib2_section3::Grib2Section3;

pub struct Grib2Section3Reader;


impl Grib2Section3Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section3, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        reader.consume(length as usize - 5);
        let section3 = Grib2Section3::new(
            length,
            section_number,
        )?;

        return Ok(section3);
    }
}
