use crate::grib2::common::grib2_error::Grib2Error;

pub struct Section6 {
    pub length: u32,
    pub section_number: u8,
    pub bitmap_indicator: u8,
    pub bitmap: Vec<u8>
}

const SECTION_NUMBER: u8 = 6;

impl Section6 {
    pub fn new(
        length: u32,
        section_number: u8,
        bitmap_indicator: u8,
        bitmap: Vec<u8>
    ) -> Result<Section6, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

        return Ok(Section6 {
            length,
            section_number,
            bitmap_indicator,
            bitmap
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section6::section6::Section6;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section6::new(
            0,
            0,
            0,
            vec![]
        );

        assert!(result.is_err());
    }
}
