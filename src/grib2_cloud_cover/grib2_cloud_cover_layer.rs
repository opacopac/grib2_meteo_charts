use crate::grib2_section0::grib2_section0::Grib2Section0;
use crate::grib2_section1::grib2_section1::Grib2Section1;
use crate::grib2_section2::grib2_section2::Grib2Section2;
use crate::grib2_section3::grib2_section3::Grib2Section3;


pub struct Grib2CloudCoverLayer {
    pub section0: Grib2Section0,
    pub section1: Grib2Section1,
    pub section2: Grib2Section2,
    pub section3: Grib2Section3,
}


impl Grib2CloudCoverLayer {
    pub fn new(
        section0: Grib2Section0,
        section1: Grib2Section1,
        section2: Grib2Section2,
        section3: Grib2Section3
    ) -> Grib2CloudCoverLayer {
        return Grib2CloudCoverLayer {
            section0,
            section1,
            section2,
            section3
        };
    }
}
