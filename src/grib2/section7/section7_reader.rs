use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section7::section7::Section7;

pub struct Section7Reader;


impl Section7Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section7, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let num_data_points = ((length - 5) / 2) as usize; // TODO: dependent on bits in sect 5
        let data_points = Section7Reader::read_data_points(reader, num_data_points)?;
        let section7 = Section7::new(
            length,
            section_number,
            data_points
        )?;

        return Ok(section7);
    }


    fn read_data_points(reader: &mut BufReader<File>, num_data_points: usize) -> Result<Vec<u16>, Grib2Error> {
        let mut buf: Vec<u16> = vec![0; num_data_points];

        reader.read_u16_into::<BigEndian>(&mut buf)?;

        return Ok(buf);
    }
}
