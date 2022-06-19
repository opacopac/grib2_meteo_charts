use std::io::Read;

use crate::grib2::common::grib2_error::Grib2Error;

pub struct ByteReader;


impl ByteReader {
    pub fn read_n_bytes(reader: &mut impl Read, size: usize) -> Result<Vec<u8>, Grib2Error> {
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;

        return Ok(buf);
    }
}
