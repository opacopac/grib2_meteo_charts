use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::scale_factor_value::ScaleFactorValue;

pub struct ScaleFactorValueReader;


impl ScaleFactorValueReader {
    pub fn read(reader: &mut impl Read) -> Result<ScaleFactorValue, Grib2Error> {
        let factor = reader.read_u8()?;
        let value = reader.read_u32::<BigEndian>()?;
        let scale_factor_value = ScaleFactorValue::new(factor, value);

        return Ok(scale_factor_value);
    }
}
