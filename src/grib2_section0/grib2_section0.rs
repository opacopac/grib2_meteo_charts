use std::error::Error;
use simple_error::bail;

use crate::grib2_section0::grib2_discipline::Grib2Discipline;

pub struct Grib2Section0 {
    pub magic: String,
    pub discipline: Grib2Discipline,
    pub edition: u8
}

const GRIB2_MAGIC: &str = "GRIB";

impl Grib2Section0 {
    pub fn new(
        magic: String,
        discipline: Grib2Discipline,
        edition: u8
    ) -> Result<Grib2Section0, Box<dyn Error>> {
        if magic != GRIB2_MAGIC {
            bail!("Invalid magic");
        }

        return Ok(Grib2Section0 {
            magic,
            discipline,
            edition
        });
    }
}
