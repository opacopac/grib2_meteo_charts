use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;
use crate::grib2_section3::grib2_grid_definition_template::Grib2GridDefinitionTemplate;
use crate::grib2_section3::grib2_optional_point_interpretation::Grib2OptionalPointInterpretation;
use crate::grib2_section3::grib2_section3::Grib2Section3;
use crate::grib2_section3::grib2_section3_template_3_0_reader::Grib2Section3Template3_0Reader;

pub struct Grib2Section3Reader;


impl Grib2Section3Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section3, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let grid_definition_source = Grib2Section3Reader::read_grid_definition_source(reader)?;
        let number_of_datapoints = reader.read_u32::<BigEndian>()?;
        let optional_point_length = reader.read_u8()?;
        let optional_point_interpretation = Grib2Section3Reader::read_optional_point_interpretation(reader)?;
        let grid_definition_template = Grib2Section3Reader::read_grid_definition_template(reader)?;
        reader.consume(length as usize - 5);
        let section3 = Grib2Section3::new(
            length,
            section_number,
            grid_definition_source,
            number_of_datapoints,
            optional_point_length,
            optional_point_interpretation,
            grid_definition_template
        )?;

        return Ok(section3);
    }


    fn read_grid_definition_source(reader: &mut BufReader<File>) -> Result<Grib2GridDefinitionSource, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let grid_def_source = match value {
            0 => Grib2GridDefinitionSource::GridDefinitionTemplate,
            1 => Grib2GridDefinitionSource::PredeterminedGridDefinition,
            255 => Grib2GridDefinitionSource::None,
            _ => Grib2GridDefinitionSource::Unknown(value)
        };

        return Ok(grid_def_source);
    }


    fn read_optional_point_interpretation(reader: &mut BufReader<File>) -> Result<Grib2OptionalPointInterpretation, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let opt_point_interpretation = match value {
            0 => Grib2OptionalPointInterpretation::None,
            1 => Grib2OptionalPointInterpretation::FullCoordinateCircles,
            2 => Grib2OptionalPointInterpretation::CoordinateLines,
            3 => Grib2OptionalPointInterpretation::ActualLatitude,
            255 => Grib2OptionalPointInterpretation::Missing,
            _ => Grib2OptionalPointInterpretation::Unknown(value)
        };

        return Ok(opt_point_interpretation);
    }


    fn read_grid_definition_template(reader: &mut BufReader<File>) -> Result<Grib2GridDefinitionTemplate, Box<dyn Error>> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let grid_def_tpl_type = match tpl_number {
            0 => {
                let tpl_3_0 = Grib2Section3Template3_0Reader::read(reader)?;
                Grib2GridDefinitionTemplate::LatLon(tpl_3_0)
            },
            65535 => Grib2GridDefinitionTemplate::Missing,
            _ => Grib2GridDefinitionTemplate::Unknown(tpl_number)
        };

        return Ok(grid_def_tpl_type);
    }
}
