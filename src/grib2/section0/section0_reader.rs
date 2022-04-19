use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::string_reader::StringReader;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section0::section0::Section0;

pub struct Section0Reader;


impl Section0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section0, Grib2Error> {
        let magic = StringReader::read_4_chars(reader)?;
        reader.seek_relative(2)?; // 2 reserved bytes
        let discipline = Section0Reader::read_discipline(reader)?;
        let edition = reader.read_u8()?;
        let length = reader.read_u64::<BigEndian>()?;

        let section0 = Section0::new(
            magic,
            discipline,
            edition,
            length
        )?;

        return Ok(section0);
    }


    fn read_discipline(reader: &mut BufReader<File>) -> Result<Discipline, Grib2Error> {
        let value = reader.read_u8()?;
        let discipline = match value {
            0 => Discipline::Meteorological,
            1 => Discipline::Hydrological,
            2 => Discipline::LandSurface,
            3 => Discipline::SatelliteRemoteSensing,
            4 => Discipline::SpaceWeather,
            10 => Discipline::Oceanographic,
            255 => Discipline::Missing,
            _ => Discipline::Unknown(value)
        };

        return Ok(discipline);
    }
}
