use std::error::Error;

pub struct Section5 {
    pub length: u32,
    pub section_number: u8,
}


impl Section5 {
    pub fn new(
        length: u32,
        section_number: u8,
    ) -> Result<Section5, Box<dyn Error>> {
        return Ok(Section5 {
            length,
            section_number,
        });
    }
}
