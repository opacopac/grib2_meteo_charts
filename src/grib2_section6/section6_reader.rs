use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section6::section6::Section6;

pub struct Section6Reader;


impl Section6Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section6, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let bitmap_indicator = reader.read_u8()?;
        let section6 = Section6::new(
            length,
            section_number,
            bitmap_indicator
        )?;

        reader.seek_relative(length as i64 - 6)?; // TODO: temp

        return Ok(section6);
    }
}
