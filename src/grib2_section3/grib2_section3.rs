use std::error::Error;

pub struct Grib2Section3 {
    pub length: u32,
    pub section_number: u8,
}


impl Grib2Section3 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Grib2Section3, Box<dyn Error>> {
        return Ok(Grib2Section3 {
            length,
            section_number,
        });
    }
}
