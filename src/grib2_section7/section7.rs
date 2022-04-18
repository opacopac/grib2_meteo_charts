use crate::grib2_common::grib2_error::Grib2Error;

pub struct Section7 {
    pub length: u32,
    pub section_number: u8,
}


const SECTION_NUMBER: u8 = 7;

impl Section7 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section7, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section7 {
            length,
            section_number,
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2_section7::section7::Section7;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section7::new(
            0,
            0
        );

        assert!(result.is_err());
    }
}