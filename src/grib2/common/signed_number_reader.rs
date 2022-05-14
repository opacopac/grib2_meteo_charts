use std::io::{BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::signed_number::SignedNumber;

pub struct SignedNumberReader;


impl SignedNumberReader {
    pub fn read<T: Read>(reader: &mut BufReader<T>) -> Result<i16, Grib2Error> {
        let raw_value = reader.read_u16::<BigEndian>()?;
        let signed_number = SignedNumber::from_u16(raw_value);

        return Ok(signed_number);
    }
}
