use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::grib2_error::Grib2Error;
use crate::grib2_section5::section5::Section5;

pub struct Section5Reader;


impl Section5Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section5, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let data_points = reader.read_u32::<BigEndian>()?;
        let section5 = Section5::new(
            length,
            section_number,
            data_points
        )?;

        reader.seek_relative(length as i64 - 9)?; // TODO: temp

        return Ok(section5);
    }
}
