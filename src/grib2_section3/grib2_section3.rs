use std::error::Error;

use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;
use crate::grib2_section3::grib2_grid_definition_template::Grib2GridDefinitionTemplate;
use crate::grib2_section3::grib2_grid_definition_template_type::Grib2GridDefinitionTemplateType;
use crate::grib2_section3::grib2_optional_point_interpretation::Grib2OptionalPointInterpretation;

pub struct Grib2Section3 {
    pub length: u32,
    pub section_number: u8,
    pub grid_definition_source: Grib2GridDefinitionSource,
    pub number_of_datapoints: u32,
    pub optional_point_length: u8,
    pub optional_point_interpretation: Grib2OptionalPointInterpretation,
    pub grid_definition_template_type: Grib2GridDefinitionTemplateType,
    pub grid_definition_template: Grib2GridDefinitionTemplate
}


impl Grib2Section3 {
    pub fn new(
        length: u32,
        section_number: u8,
        grid_definition_source: Grib2GridDefinitionSource,
        number_of_datapoints: u32,
        optional_point_length: u8,
        optional_point_interpretation: Grib2OptionalPointInterpretation,
        grid_definition_template_type: Grib2GridDefinitionTemplateType,
        grid_definition_template: Grib2GridDefinitionTemplate
    ) -> Result<Grib2Section3, Box<dyn Error>> {
        return Ok(Grib2Section3 {
            length,
            section_number,
            grid_definition_source,
            number_of_datapoints,
            optional_point_length,
            optional_point_interpretation,
            grid_definition_template_type,
            grid_definition_template
        });
    }
}
