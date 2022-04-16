pub struct Grib2Section0 {
    grib_magic: String,
    /*reserved: [u8; 2],
    discipline: u8,
    edition: u8,
    tot_length: [u8; 8]*/
}


impl Grib2Section0 {
    pub fn new(
        grib_magic: String
    ) -> Grib2Section0 {
        return Grib2Section0 {
            grib_magic
        }
    }
}
