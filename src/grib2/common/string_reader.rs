use std::fs::File;
use std::io::{BufReader, Read};
use std::str::from_utf8;

use crate::grib2::common::grib2_error::Grib2Error;

pub struct StringReader;


impl StringReader {
    pub fn read_4_chars(reader: &mut BufReader<File>) -> Result<String, Grib2Error> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;

        let text = from_utf8(&buf)?.to_string();

        return Ok(text);
    }
}
