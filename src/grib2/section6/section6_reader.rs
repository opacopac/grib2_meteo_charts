use std::fs::File;
use std::io::{BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section6::section6::Section6;

pub struct Section6Reader;


impl Section6Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section6, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let bitmap_indicator = reader.read_u8()?;
        let bitmap = Section6Reader::read_bitmap(reader, (length - 6) as usize)?;

        let section6 = Section6::new(
            length,
            section_number,
            bitmap_indicator,
            bitmap
        )?;

        return Ok(section6);
    }


    fn read_bitmap(reader: &mut BufReader<File>, size: usize) -> Result<Vec<u8>, Grib2Error> {
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;

        return Ok(buf);
    }
}
