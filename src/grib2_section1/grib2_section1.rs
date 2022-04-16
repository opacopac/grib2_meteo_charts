use std::error::Error;


pub struct Grib2Section1 {
    pub length: u32,
    pub section_number: u8,
    pub center: u16,
    pub subcenter: u16,
    pub master_table_version: u8,
    pub local_table_version: u8,
}


impl Grib2Section1 {
    pub fn new(
        length: u32,
        section_number: u8,
        center: u16,
        subcenter: u16,
        master_table_version: u8,
        local_table_version: u8
    ) -> Result<Grib2Section1, Box<dyn Error>> {
        return Ok(Grib2Section1 {
            length,
            section_number,
            center,
            subcenter,
            master_table_version,
            local_table_version
        });
    }
}
