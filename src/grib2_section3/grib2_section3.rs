use std::error::Error;
use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;

pub struct Grib2Section3 {
    pub length: u32,
    pub section_number: u8,
    pub grid_definition_source: Grib2GridDefinitionSource
}


impl Grib2Section3 {
    pub fn new(
        length: u32,
        section_number: u8,
        grid_definition_source: Grib2GridDefinitionSource
    ) -> Result<Grib2Section3, Box<dyn Error>> {
        return Ok(Grib2Section3 {
            length,
            section_number,
            grid_definition_source
        });
    }
}
