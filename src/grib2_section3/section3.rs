use std::error::Error;

use crate::grib2_section3::grid_definition_source::GridDefinitionSource;
use crate::grib2_section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2_section3::optional_point_interpretation::OptionalPointInterpretation;

pub struct Section3 {
    pub length: u32,
    pub section_number: u8,
    pub grid_definition_source: GridDefinitionSource,
    pub number_of_datapoints: u32,
    pub optional_point_length: u8,
    pub optional_point_interpretation: OptionalPointInterpretation,
    pub grid_definition_template: GridDefinitionTemplate,
}


impl Section3 {
    pub fn new(
        length: u32,
        section_number: u8,
        grid_definition_source: GridDefinitionSource,
        number_of_datapoints: u32,
        optional_point_length: u8,
        optional_point_interpretation: OptionalPointInterpretation,
        grid_definition_template: GridDefinitionTemplate,
    ) -> Result<Section3, Box<dyn Error>> {
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
