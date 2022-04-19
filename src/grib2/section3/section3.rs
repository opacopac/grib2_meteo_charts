use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section3::grid_definition_source::GridDefinitionSource;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;

pub struct Section3 {
    pub length: u32,
    pub section_number: u8,
    pub grid_definition_source: GridDefinitionSource,
    pub number_of_datapoints: u32,
    pub optional_point_length: u8,
    pub optional_point_interpretation: OptionalPointInterpretation,
    pub grid_definition_template: GridDefinitionTemplate,
}


const SECTION_NUMBER: u8 = 3;

impl Section3 {
    pub fn new(
        length: u32,
        section_number: u8,
        grid_definition_source: GridDefinitionSource,
        number_of_datapoints: u32,
        optional_point_length: u8,
        optional_point_interpretation: OptionalPointInterpretation,
        grid_definition_template: GridDefinitionTemplate,
    ) -> Result<Section3, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section3 {
            length,
            section_number,
            grid_definition_source,
            number_of_datapoints,
            optional_point_length,
            optional_point_interpretation,
            grid_definition_template,
        });
    }
}




#[cfg(test)]
mod tests {
    use crate::grib2::section3::grid_definition_source::GridDefinitionSource;
    use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
    use crate::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;
    use crate::grib2::section3::section3::Section3;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section3::new(
            0,
            0,
            GridDefinitionSource::None,
            0,
            0,
            OptionalPointInterpretation::None,
            GridDefinitionTemplate::Missing
        );

        assert!(result.is_err());
    }
}
