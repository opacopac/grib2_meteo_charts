use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::signed_number::SignedNumber;

pub struct SignedNumberReader;


impl SignedNumberReader {
    pub fn read(reader: &mut impl Read) -> Result<i16, Grib2Error> {
        let raw_value = reader.read_u16::<BigEndian>()?;
        let signed_number = SignedNumber::from_u16(raw_value);

        return Ok(signed_number);
    }
}
