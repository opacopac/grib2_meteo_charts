use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};
use crate::grib2_section2::section2::Section2;


pub struct Section2Reader;


impl Section2Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section2, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        reader.consume(length as usize - 5);
        let section2 = Section2::new(
            length,
            section_number,
        )?;

        return Ok(section2);
    }
}
