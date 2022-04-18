use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section4::section4::Section4;

pub struct Section4Reader;


impl Section4Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section4, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;

        reader.consume(length as usize - 5); // TODO: temp

        let section4 = Section4::new(
            length,
            section_number,
        )?;

        return Ok(section4);
    }
}
