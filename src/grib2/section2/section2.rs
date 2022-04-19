use crate::grib2::common::grib2_error::Grib2Error;

pub struct Section2 {
    pub length: u32,
    pub section_number: u8,
}

const SECTION_NUMBER: u8 = 2;

impl Section2 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section2, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section2 {
            length,
            section_number,
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section2::section2::Section2;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section2::new(
            0,
            0
        );

        assert!(result.is_err());
    }
}
