use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section1::grib2_section1::Grib2Section1;

pub struct Grib2Section1Reader;


impl Grib2Section1Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section1, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let center = reader.read_u16::<BigEndian>()?;
        let subcenter = reader.read_u16::<BigEndian>()?;
        let master_table_version = reader.read_u8()?;
        let local_table_version = reader.read_u8()?;
        let section1 = Grib2Section1::new(
            length,
            section_number,
            center,
            subcenter,
            master_table_version,
            local_table_version
        )?;

        return Ok(section1);
    }
}
