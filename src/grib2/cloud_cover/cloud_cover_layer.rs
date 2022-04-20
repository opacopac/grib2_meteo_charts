use crate::grib2::section0::section0::Section0;
use crate::grib2::section1::section1::Section1;
use crate::grib2::section2::section2::Section2;
use crate::grib2::section3::section3::Section3;
use crate::grib2::section4::section4::Section4;
use crate::grib2::section5::section5::Section5;
use crate::grib2::section6::section6::Section6;
use crate::grib2::section7::section7::Section7;
use crate::grib2::section8::section8::Section8;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate::GridPointDataSimplePacking;

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


    pub fn get_value(&self, index: usize) -> f32 {
        let raw_value = self.section7.data_points[index];
        println!("raw value {:?}", raw_value);

        match &self.section5.data_representation_template {
            GridPointDataSimplePacking(tpl) => {
                println!("{:?}", tpl);
                let c1 = (2 as f32).powi(tpl.binary_scale_factor_e as i32);
                let c2 = (10 as f32).powi(tpl.decimal_scale_factor_d as i32);
                /*let c1 = Math.pow(2, section5.data.dataRepresentationTemplate.E)
                let c2 = Math.pow(10, section5.data.dataRepresentationTemplate.D)*/

                return (tpl.reference_value + raw_value as f32 * c1) as f32 / c2;
            }
            _ => { panic!("invalid template") } // TODO: temp
        }

        // return (self.section5.data_representation_template.data.dataRepresentationTemplate.R + rawValue * c1) / c2;
    }
}
