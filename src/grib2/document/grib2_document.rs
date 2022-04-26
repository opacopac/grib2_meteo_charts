use derive_new::new;

use crate::grib2::section0::section0::Section0;
use crate::grib2::section1::section1::Section1;
use crate::grib2::section2::section2::Section2;
use crate::grib2::section3::section3::Section3;
use crate::grib2::section4::section4::Section4;
use crate::grib2::section5::section5::Section5;
use crate::grib2::section6::section6::Section6;
use crate::grib2::section7::section7::Section7;
use crate::grib2::section8::section8::Section8;

#[derive(new)]
pub struct Grib2Document {
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
