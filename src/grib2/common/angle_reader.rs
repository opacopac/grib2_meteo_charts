use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;

pub struct AngleReader;


const ANGLE_FACTOR: f32 = 1000000.0;

impl AngleReader {
    pub fn read_lat_lon(reader: &mut BufReader<File>) -> Result<(f32, f32), Grib2Error> {
        let lat = AngleReader::read_angle(reader)?;
        let lon = AngleReader::read_angle(reader)?;

        return Ok((lat, lon));
    }


    pub fn read_angle(reader: &mut BufReader<File>) -> Result<f32, Grib2Error> {
        let value = reader.read_u32::<BigEndian>()? as f32 / ANGLE_FACTOR;

        return Ok(value);
    }
}
