use std::error::Error;

pub struct Section6 {
    pub length: u32,
    pub section_number: u8,
}


impl Section6 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section6, Box<dyn Error>> {
        return Ok(Section6 {
            length,
            section_number,
        });
    }
}
