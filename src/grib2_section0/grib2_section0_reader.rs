use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::from_utf8;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section0::grib2_discipline::Grib2Discipline;
use crate::grib2_section0::grib2_section0::Grib2Section0;

pub struct Grib2Section0Reader;


impl Grib2Section0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section0, Box<dyn Error>> {
        let magic = Grib2Section0Reader::read_magic(reader)?;
        reader.consume(2); // 2 reserved bytes
        let discipline = Grib2Section0Reader::read_discipline(reader)?;
        let edition = reader.read_u8()?;
        let length = reader.read_u64::<BigEndian>()?;

        let section0 = Grib2Section0::new(
            magic,
            discipline,
            edition,
            length
        )?;

        return Ok(section0);
    }


    fn read_magic(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;

        let magic = from_utf8(&buf)?.to_string();

        return Ok(magic);
    }


    fn read_discipline(reader: &mut BufReader<File>) -> Result<Grib2Discipline, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let discipline = match value {
            0 => Grib2Discipline::Meteorological,
            1 => Grib2Discipline::Hydrological,
            2 => Grib2Discipline::LandSurface,
            3 => Grib2Discipline::SatelliteRemoteSensing,
            4 => Grib2Discipline::SpaceWeather,
            10 => Grib2Discipline::Oceanographic,
            255 => Grib2Discipline::Missing,
            _ => Grib2Discipline::Unknown(value)
        };

        return Ok(discipline);
    }
}
