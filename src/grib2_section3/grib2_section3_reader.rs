use byteorder::{BigEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;

use crate::grib2_section3::grib2_section3::Grib2Section3;

pub struct Grib2Section3Reader;


impl Grib2Section3Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section3, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let grid_definition_source = Grib2Section3Reader::read_grid_definition_source(reader)?;
        reader.consume(length as usize - 5);
        let section3 = Grib2Section3::new(
            length,
            section_number,
            grid_definition_source
        )?;

        return Ok(section3);
    }


    fn read_grid_definition_source(reader: &mut BufReader<File>) -> Result<Grib2GridDefinitionSource, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let ref_time_significance = match value {
            0 => Grib2GridDefinitionSource::GridDefinitionTemplate,
            1 => Grib2GridDefinitionSource::PredeterminedGridDefinition,
            255 => Grib2GridDefinitionSource::None,
            _ => Grib2GridDefinitionSource::Unknown(value)
        };

        return Ok(ref_time_significance);
    }
}
