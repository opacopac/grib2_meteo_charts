use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section7::section7::Section7;

pub struct Section7Reader;


impl Section7Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section7, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let section7 = Section7::new(
            length,
            section_number,
        )?;

        reader.seek_relative(length as i64 - 5)?; // TODO: temp

        return Ok(section7);
    }
}
