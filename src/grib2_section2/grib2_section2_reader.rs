use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};
use crate::grib2_section2::grib2_section2::Grib2Section2;


pub struct Grib2Section2Reader;


impl Grib2Section2Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section2, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        reader.consume(length as usize - 5);
        let section2 = Grib2Section2::new(
            length,
            section_number,
        )?;

        return Ok(section2);
    }
}
