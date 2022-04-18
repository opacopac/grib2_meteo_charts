use crate::grib2_section0::section0::Section0;
use crate::grib2_section1::section1::Section1;
use crate::grib2_section2::section2::Section2;
use crate::grib2_section3::section3::Section3;
use crate::grib2_section4::section4::Section4;
use crate::grib2_section5::section5::Section5;

pub struct CloudCoverLayer {
    pub section0: Section0,
    pub section1: Section1,
    pub section2: Section2,
    pub section3: Section3,
    pub section4: Section4,
    pub section5: Section5
}


impl CloudCoverLayer {
    pub fn new(
        section0: Section0,
        section1: Section1,
        section2: Section2,
        section3: Section3,
        section4: Section4,
        section5: Section5
    ) -> CloudCoverLayer {
        return CloudCoverLayer {
            section0,
            section1,
            section2,
            section3,
            section4,
            section5
        };
    }
}
