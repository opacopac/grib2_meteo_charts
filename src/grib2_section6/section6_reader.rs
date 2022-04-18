use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section6::section6::Section6;

pub struct Section6Reader;


impl Section6Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section6, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;

        reader.consume(length as usize - 5); // TODO: temp

        let section6 = Section6::new(
            length,
            section_number,
        )?;

        return Ok(section6);
    }
}
