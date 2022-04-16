use crate::grib2_section0::grib2_section0::Grib2Section0;
use crate::grib2_section1::grib2_section1::Grib2Section1;


pub struct Grib2CloudCoverLayer {
    pub section0: Grib2Section0,
    pub section1: Grib2Section1,
}


impl Grib2CloudCoverLayer {
    pub fn new(
        section0: Grib2Section0,
        section1: Grib2Section1
    ) -> Grib2CloudCoverLayer {
        return Grib2CloudCoverLayer {
            section0,
            section1
        };
    }
}
