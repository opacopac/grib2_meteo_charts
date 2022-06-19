use std::io::Read;

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

pub struct ShapeOfEarthReader;


impl ShapeOfEarthReader {
    pub fn read(reader: &mut impl Read) -> Result<ShapeOfEarth, Grib2Error> {
        let value = reader.read_u8()?;
        let shape_of_earth = match value {
            6 => ShapeOfEarth::SphericalRadius6371229,
            255 => ShapeOfEarth::Missing,
            _ => ShapeOfEarth::Unknown(value)
        };

        return Ok(shape_of_earth);
    }
}
