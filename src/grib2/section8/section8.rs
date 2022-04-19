use crate::grib2::common::grib2_error::Grib2Error;

pub struct Section8 {
    pub end_magic: String
}


const GRIB2_END_MAGIC: &str = "7777";

impl Section8 {
    pub fn new(end_magic: String) -> Result<Section8, Grib2Error> {
        if end_magic != GRIB2_END_MAGIC {
            return Err(Grib2Error::InvalidData(
                format!("invalid end section {}, expected: {}", end_magic, GRIB2_END_MAGIC)
            ));
        }

        return Ok(Section8 { end_magic });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section8::section8::Section8;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section8::new("MEEP".to_string());

        assert!(result.is_err());
    }
}
