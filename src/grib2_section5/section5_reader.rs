use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section5::section5::Section5;

pub struct Section5Reader;


impl Section5Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section5, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let section5 = Section5::new(
            length,
            section_number,
        )?;

        reader.seek_relative(length as i64 - 5)?; // TODO: temp

        return Ok(section5);
    }
}
