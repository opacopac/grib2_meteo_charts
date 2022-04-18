use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::lat_lon::LatLon;

pub struct AngleReader;


const ANGLE_FACTOR: f32 = 1000000.0;

impl AngleReader {
    pub fn read_lat_lon(reader: &mut BufReader<File>) -> Result<LatLon, Box<dyn Error>> {
        let lat = AngleReader::read_angle(reader)?;
        let lon = AngleReader::read_angle(reader)?;
        let lat_lon = LatLon::new(lat, lon);

        return Ok(lat_lon);
    }


    pub fn read_angle(reader: &mut BufReader<File>) -> Result<f32, Box<dyn Error>> {
        let value = reader.read_u32::<BigEndian>()? as f32 / ANGLE_FACTOR;

        return Ok(value);
    }
}
