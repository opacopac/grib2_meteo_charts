use std::error::Error;

pub struct Grib2Section2 {
    pub length: u32,
    pub section_number: u8,
}


impl Grib2Section2 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Grib2Section2, Box<dyn Error>> {
        return Ok(Grib2Section2 {
            length,
            section_number,
        });
    }
}
