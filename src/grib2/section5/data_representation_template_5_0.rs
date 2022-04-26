use derive_new::new;

use crate::grib2::section5::original_field_type::OriginalFieldType;

#[derive(Debug, new)]
pub struct DataRepresentationTemplate5_0 {
    pub reference_value: f32,
    pub binary_scale_factor_e: i16,
    pub decimal_scale_factor_d: i16,
    pub number_of_bits: u8,
    pub original_field_type: OriginalFieldType
}
