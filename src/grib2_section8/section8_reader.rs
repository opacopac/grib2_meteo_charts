use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::grib2_common::string_reader::StringReader;
use crate::grib2_section8::section8::Section8;

pub struct Section8Reader;


impl Section8Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section8, Box<dyn Error>> {
        let magic = StringReader::read_4_chars(reader)?;
        let section8 = Section8::new(magic)?;

        return Ok(section8);
    }
}
