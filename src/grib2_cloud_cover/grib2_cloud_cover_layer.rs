use crate::grib2_section0::grib2_section0::Grib2Section0;


pub struct Grib2CloudCoverLayer {
    pub section0: Grib2Section0
}


impl Grib2CloudCoverLayer {
    pub fn new(
        section0: Grib2Section0
    ) -> Grib2CloudCoverLayer {
        return Grib2CloudCoverLayer {
            section0
        };
    }
}
