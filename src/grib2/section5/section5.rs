use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate;

#[derive(Debug)]
pub struct Section5 {
    pub length: u32,
    pub section_number: u8,
    pub data_points: u32,
    pub data_representation_template: DataRepresentationTemplate
}


const SECTION_NUMBER: u8 = 5;

impl Section5 {
    pub fn new(
        length: u32,
        section_number: u8,
        data_points: u32,
        data_representation_template: DataRepresentationTemplate
    ) -> Result<Section5, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section5 {
            length,
            section_number,
            data_points,
            data_representation_template
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section5::data_representation_template::DataRepresentationTemplate;
    use crate::grib2::section5::section5::Section5;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section5::new(
            0,
            0,
            0,
            DataRepresentationTemplate::Missing
        );

        assert!(result.is_err());
    }
}
