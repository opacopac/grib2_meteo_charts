use derive_new::new;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section0::section0::Section0;
use crate::grib2::section1::section1::Section1;
use crate::grib2::section2::section2::Section2;
use crate::grib2::section3::section3::Section3;
use crate::grib2::section4::section4::Section4;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate::GridPointDataSimplePacking;
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


impl Grib2Document {
    pub fn calculate_data_points<T: Copy>(
        &self,
        missing_value: T,
        transform_fn: fn(f32) -> T
    ) -> Result<Vec<T>, Grib2Error> {
        let (ref_value, c1, c2) = match &self.section5.data_representation_template {
            GridPointDataSimplePacking(tpl) => {
                let c1 = (2 as f32).powi(tpl.binary_scale_factor_e as i32);
                let c2 = (10 as f32).powi(tpl.decimal_scale_factor_d as i32);

                (tpl.reference_value, c1, c2)
            }
            _ => return Err(Grib2Error::InvalidData("invalid data representation template".to_string()))
        };

        let bitmap = &self.section6.bitmap;
        let raw_data_points = &self.section7.data_points;

        if raw_data_points.is_empty() {
            return Ok(vec![missing_value; self.section3.number_of_datapoints as usize]);
        }

        let mut data_points: Vec<T> = vec![];
        let mut j = 0;

        for i in 0..self.section3.number_of_datapoints {
            if bitmap.is_empty() || (bitmap[(i / 8) as usize] & (0b10000000 >> (i % 8)) > 0) {
                let raw_value = raw_data_points[j] as f32;
                let data_value = (ref_value + raw_value * c1) as f32 / c2;
                let data_value_transformed = transform_fn(data_value);
                data_points.push(data_value_transformed);
                j += 1;
            } else {
                data_points.push(missing_value);
            }
        }

        return Ok(data_points);
    }
}
