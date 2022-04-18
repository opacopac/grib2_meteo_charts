use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};
use crate::grib2_common::lat_lon::LatLon;

pub struct LatLonReader;


impl LatLonReader {
    pub fn read(reader: &mut BufReader<File>) -> Result<LatLon, Box<dyn Error>> {
        let lat = reader.read_u32::<BigEndian>()? as f32 / 1000000.0;
        let lon = reader.read_u32::<BigEndian>()? as f32 / 1000000.0;
        let lat_lon = LatLon::new(lat, lon);

        return Ok(lat_lon);
    }
}
