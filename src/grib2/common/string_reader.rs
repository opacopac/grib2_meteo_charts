use crate::grib2::common::grib2_error::Grib2Error;
use std::io::Read;
use std::str::from_utf8;


pub struct StringReader;


impl StringReader {
    pub fn read_n_chars(reader: &mut impl Read, size: usize) -> Result<String, Grib2Error> {
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;

        let text = from_utf8(&buf)?.to_string();

        return Ok(text);
    }
}
