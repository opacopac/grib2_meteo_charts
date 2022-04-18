use std::error::Error;

pub struct Section7 {
    pub length: u32,
    pub section_number: u8,
}


impl Section7 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section7, Box<dyn Error>> {
        return Ok(Section7 {
            length,
            section_number,
        });
    }
}
