use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;

pub struct Section4 {
    pub length: u32,
    pub section_number: u8,
    pub coordinate_values: u16,
    pub product_definition_template: ProductDefinitionTemplate
}


const SECTION_NUMBER: u8 = 4;

impl Section4 {
    pub fn new(
        length: u32,
        section_number: u8,
        coordinate_values: u16,
        product_definition_template: ProductDefinitionTemplate
    ) -> Result<Section4, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section4 {
            length,
            section_number,
            coordinate_values,
            product_definition_template
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;
    use crate::grib2::section4::section4::Section4;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section4::new(
            0,
            0,
            0,
            ProductDefinitionTemplate::Missing
        );

        assert!(result.is_err());
    }
}
