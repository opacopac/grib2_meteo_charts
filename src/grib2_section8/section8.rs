use std::error::Error;

pub struct Section8 {
    pub end_magic: String
}


impl Section8 {
    pub fn new(end_magic: String) -> Result<Section8, Box<dyn Error>> {
        return Ok(Section8 { end_magic });
    }
}
