use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section3::grid_definition_source::GridDefinitionSource;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;
use crate::grib2::section3::section3::Section3;
use crate::grib2::section3::section3_template_3_0_reader::Section3Template3_0Reader;
use crate::grib2::section3::section3_template_3_101_reader::Section3Template3_101Reader;

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
            101 => {
                let tpl_3_101 = Section3Template3_101Reader::read(reader)?;
                GridDefinitionTemplate::UnstructuredGrid(tpl_3_101)
            }
            _ => return Err(Grib2Error::InvalidData(
                format!("unsupported grid definition template: {}", tpl_number)
            ))
        };

        return Ok(grid_def_tpl_type);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};
    use crate::grib2::section3::grid_definition_source::GridDefinitionSource;
    use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
    use crate::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;
    use crate::grib2::section3::section3_reader::Section3Reader;

    #[test]
    fn it_correctly_parses_a_section3_with_a_regular_lat_lon_grid() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x48, 0x03, 0x00, 0x00, 0x0D, 0xD4, 0x96, 0x00, 0x00, 0x00, 0x00, 0x06, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
            0x04, 0xBF, 0x00, 0x00, 0x02, 0xEA, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x02, 0x92,
            0xDF, 0xE0, 0x15, 0x39, 0x0B, 0x60, 0x30, 0x03, 0x76, 0x3B, 0x00, 0x01, 0x36, 0x5D, 0x20, 0x00,
            0x00, 0x4E, 0x20, 0x00, 0x00, 0x4E, 0x20, 0x40
        ]));

        let result = Section3Reader::read(&mut reader);
        assert!(result.is_ok());

        let section3 = result.unwrap();
        assert_eq!(72, section3.length);
        assert_eq!(3, section3.section_number);
        assert_eq!(GridDefinitionSource::GridDefinitionTemplate, section3.grid_definition_source);
        assert_eq!(906390, section3.number_of_datapoints);
        assert_eq!(0, section3.optional_point_length);
        assert_eq!(OptionalPointInterpretation::None, section3.optional_point_interpretation);

        match section3.grid_definition_template {
            GridDefinitionTemplate::LatitudeLongitude(_tpl) => {},
            _ => panic!("wrong grid definition template")
        };
    }


    #[test]
    fn it_correctly_parses_a_section3_with_an_unstructured_grid() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x23, 0x03, 0x00, 0x00, 0x2D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x06, 0x00,
            0x00, 0x1A, 0x01, 0xA2, 0x7B, 0x8D, 0xE6, 0x18, 0xC4, 0x11, 0xE4, 0x82, 0x0A, 0xB5, 0xB0, 0x98,
            0xC6, 0xA5, 0xC0
        ]));

        let result = Section3Reader::read(&mut reader);
        assert!(result.is_ok());

        let section3 = result.unwrap();
        assert_eq!(35, section3.length);
        assert_eq!(3, section3.section_number);
        assert_eq!(GridDefinitionSource::GridDefinitionTemplate, section3.grid_definition_source);
        assert_eq!(2949120, section3.number_of_datapoints);
        assert_eq!(0, section3.optional_point_length);
        assert_eq!(OptionalPointInterpretation::None, section3.optional_point_interpretation);

        match section3.grid_definition_template {
            GridDefinitionTemplate::UnstructuredGrid(_tpl) => {},
            _ => panic!("wrong grid definition template {:?}", section3.grid_definition_template)
        };
    }
}
