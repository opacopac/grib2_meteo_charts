use std::error::Error;

pub struct Section4 {
    pub length: u32,
    pub section_number: u8,
}


impl Section4 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section4, Box<dyn Error>> {
        return Ok(Section4 {
            length,
            section_number,
        });
    }
}
