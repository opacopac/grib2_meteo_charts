use crate::grib2_section0::section0::Section0;
use crate::grib2_section1::section1::Section1;
use crate::grib2_section2::section2::Section2;
use crate::grib2_section3::section3::Section3;
use crate::grib2_section4::section4::Section4;
use crate::grib2_section5::section5::Section5;
use crate::grib2_section6::section6::Section6;
use crate::grib2_section7::section7::Section7;
use crate::grib2_section8::section8::Section8;

pub struct CloudCoverLayer {
    pub section0: Section0,
    pub section1: Section1,
    pub section2: Section2,
    pub section3: Section3,
    pub section4: Section4,
    pub section5: Section5,
    pub section6: Section6,
    pub section7: Section7,
    pub section8: Section8
}


impl CloudCoverLayer {
    pub fn new(
        section0: Section0,
        section1: Section1,
        section2: Section2,
        section3: Section3,
        section4: Section4,
        section5: Section5,
        section6: Section6,
        section7: Section7,
        section8: Section8
    ) -> CloudCoverLayer {
        return CloudCoverLayer {
            section0,
            section1,
            section2,
            section3,
            section4,
            section5,
            section6,
            section7,
            section8
        };
    }
}
