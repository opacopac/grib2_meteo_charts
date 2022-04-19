use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section2::section2::Section2;


pub struct Section2Reader;


impl Section2Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section2, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let section2 = Section2::new(
            length,
            section_number,
        )?;

        reader.seek_relative(length as i64 - 5)?;

        return Ok(section2);
    }
}
