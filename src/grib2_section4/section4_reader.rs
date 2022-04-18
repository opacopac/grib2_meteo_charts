use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::grib2_error::Grib2Error;
use crate::grib2_section4::section4::Section4;

pub struct Section4Reader;


impl Section4Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section4, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let section4 = Section4::new(
            length,
            section_number,
        )?;

        reader.seek_relative(length as i64 - 5)?; // TODO: temp

        return Ok(section4);
    }
}
