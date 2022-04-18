use std::error::Error;

use simple_error::bail;

pub struct Section8 {
    pub end_magic: String
}


const GRIB2_END_MAGIC: &str = "7777";

impl Section8 {
    pub fn new(end_magic: String) -> Result<Section8, Box<dyn Error>> {
        if end_magic != GRIB2_END_MAGIC {
            bail!("Invalid end section");
        }

        return Ok(Section8 { end_magic });
    }
}
