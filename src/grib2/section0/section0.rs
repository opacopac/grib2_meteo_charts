use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section0::discipline::Discipline;

pub struct Section0 {
    pub magic: String,
    pub discipline: Discipline,
    pub edition: u8,
    pub length: u64
}

const GRIB2_MAGIC: &str = "GRIB";

impl Section0 {
    pub fn new(
        magic: String,
        discipline: Discipline,
        edition: u8,
        length: u64
    ) -> Result<Section0, Grib2Error> {
        if magic != GRIB2_MAGIC {
            return Err(Grib2Error::InvalidData(
                format!("Invalid magic {}, expected: {}", magic, GRIB2_MAGIC)
            ));
        }

        return Ok(Section0 {
            magic,
            discipline,
            edition,
            length
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section0::discipline::Discipline;
    use crate::grib2::section0::section0::Section0;

    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section0::new(
            "MEEP".to_string(),
            Discipline::Missing,
            0,
            0
        );

        assert!(result.is_err());
    }
}
