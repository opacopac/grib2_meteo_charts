use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section3::grid_definition_source::GridDefinitionSource;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;
use crate::grib2::section3::section3::Section3;
use crate::grib2::section3::section3_template_3_0_reader::Section3Template3_0Reader;

pub struct Section3Reader;


impl Section3Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Section3, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let grid_definition_source = Section3Reader::read_grid_definition_source(reader)?;
        let number_of_datapoints = reader.read_u32::<BigEndian>()?;
        let optional_point_length = reader.read_u8()?;
        let optional_point_interpretation = Section3Reader::read_optional_point_interpretation(reader)?;
        let grid_definition_template = Section3Reader::read_grid_definition_template(reader)?;
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


    fn read_grid_definition_source<T: Read>(reader: &mut BufReader<T>) -> Result<GridDefinitionSource, Grib2Error> {
        let value = reader.read_u8()?;
        let grid_def_source = match value {
            0 => GridDefinitionSource::GridDefinitionTemplate,
            1 => GridDefinitionSource::PredeterminedGridDefinition,
            255 => GridDefinitionSource::None,
            _ => GridDefinitionSource::Unknown(value)
        };

        return Ok(grid_def_source);
    }


    fn read_optional_point_interpretation<T: Read>(reader: &mut BufReader<T>) -> Result<OptionalPointInterpretation, Grib2Error> {
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


    fn read_grid_definition_template<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<GridDefinitionTemplate, Grib2Error> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let grid_def_tpl_type = match tpl_number {
            0 => {
                let tpl_3_0 = Section3Template3_0Reader::read(reader)?;
                GridDefinitionTemplate::LatitudeLongitude(tpl_3_0)
            },
            65535 => GridDefinitionTemplate::Missing,
            _ => GridDefinitionTemplate::Unknown(tpl_number)
        };

        return Ok(grid_def_tpl_type);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};
    use crate::grib2::section3::section3_reader::Section3Reader;

    #[test]
    fn it_correctly_parses_a_section0() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x48, 0x03, 0x00, 0x00, 0x0D, 0xD4, 0x96, 0x00, 0x00, 0x00, 0x00, 0x06, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
            0x04, 0xBF, 0x00, 0x00, 0x02, 0xEA, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x02, 0x92,
            0xDF, 0xE0, 0x15, 0x39, 0x0B, 0x60, 0x30, 0x03, 0x76, 0x3B, 0x00, 0x01, 0x36, 0x5D, 0x20, 0x00,
            0x00, 0x4E, 0x20, 0x00, 0x00, 0x4E, 0x20, 0x40
        ]));

        let result = Section3Reader::read(&mut reader);
        assert!(result.is_ok());

        let section2 = result.unwrap();
        assert_eq!(72, section2.length);
        assert_eq!(3, section2.section_number);
    }
}
