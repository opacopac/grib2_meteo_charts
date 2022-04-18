use std::error::Error;

use simple_error::bail;

use crate::grib2_section0::discipline::Discipline;

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
    ) -> Result<Section0, Box<dyn Error>> {
        if magic != GRIB2_MAGIC {
            bail!("Invalid magic");
        }

        return Ok(Section0 {
            magic,
            discipline,
            edition,
            length
        });
    }
}
