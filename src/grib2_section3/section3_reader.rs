use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_section3::grid_definition_source::GridDefinitionSource;
use crate::grib2_section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2_section3::optional_point_interpretation::OptionalPointInterpretation;
use crate::grib2_section3::section3::Section3;
use crate::grib2_section3::section3_template_3_0_reader::Section3Template3_0Reader;

pub struct Section3Reader;


impl Section3Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section3, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let grid_definition_source = Section3Reader::read_grid_definition_source(reader)?;
        let number_of_datapoints = reader.read_u32::<BigEndian>()?;
        let optional_point_length = reader.read_u8()?;
        let optional_point_interpretation = Section3Reader::read_optional_point_interpretation(reader)?;
        let grid_definition_template = Section3Reader::read_grid_definition_template(reader)?;
        reader.consume(length as usize - 5);
        let section3 = Section3::new(
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


    fn read_grid_definition_source(reader: &mut BufReader<File>) -> Result<GridDefinitionSource, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let grid_def_source = match value {
            0 => GridDefinitionSource::GridDefinitionTemplate,
            1 => GridDefinitionSource::PredeterminedGridDefinition,
            255 => GridDefinitionSource::None,
            _ => GridDefinitionSource::Unknown(value)
        };

        return Ok(grid_def_source);
    }


    fn read_optional_point_interpretation(reader: &mut BufReader<File>) -> Result<OptionalPointInterpretation, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let opt_point_interpretation = match value {
            0 => OptionalPointInterpretation::None,
            1 => OptionalPointInterpretation::FullCoordinateCircles,
            2 => OptionalPointInterpretation::CoordinateLines,
            3 => OptionalPointInterpretation::ActualLatitude,
            255 => OptionalPointInterpretation::Missing,
            _ => OptionalPointInterpretation::Unknown(value)
        };

        return Ok(opt_point_interpretation);
    }


    fn read_grid_definition_template(reader: &mut BufReader<File>) -> Result<GridDefinitionTemplate, Box<dyn Error>> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let grid_def_tpl_type = match tpl_number {
            0 => {
                let tpl_3_0 = Section3Template3_0Reader::read(reader)?;
                GridDefinitionTemplate::LatLon(tpl_3_0)
            },
            65535 => GridDefinitionTemplate::Missing,
            _ => GridDefinitionTemplate::Unknown(tpl_number)
        };

        return Ok(grid_def_tpl_type);
    }
}
