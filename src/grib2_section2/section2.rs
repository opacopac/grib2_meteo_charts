use std::error::Error;

pub struct Section2 {
    pub length: u32,
    pub section_number: u8,
}


impl Section2 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section2, Box<dyn Error>> {
        return Ok(Section2 {
            length,
            section_number,
        });
    }
}
