use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::grib2_error::Grib2Error;
use crate::grib2_common::scale_factor_value::ScaleFactorValue;

pub struct ScaleFactorValueReader;


impl ScaleFactorValueReader {
    pub fn read(reader: &mut BufReader<File>) -> Result<ScaleFactorValue, Grib2Error> {
        let factor = reader.read_u8()?;
        let value = reader.read_u32::<BigEndian>()?;
        let scale_factor_value = ScaleFactorValue::new(factor, value);

        return Ok(scale_factor_value);
    }
}
