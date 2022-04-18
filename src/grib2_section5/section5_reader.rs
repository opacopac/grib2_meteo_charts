use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section5::section5::Section5;

pub struct Section5Reader;


impl Section5Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section5, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;

        reader.consume(length as usize - 5); // TODO: temp

        let section5 = Section5::new(
            length,
            section_number,
        )?;

        return Ok(section5);
    }
}
