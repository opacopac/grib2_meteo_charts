use crate::grib2::section5::original_field_type::OriginalFieldType;

#[derive(Debug)]
pub struct DataRepresentationTemplate5_0 {
    pub reference_value: f32,
    pub binary_scale_factor_e: i16,
    pub decimal_scale_factor_d: i16,
    pub number_of_bits: u8,
    pub original_field_type: OriginalFieldType
}


impl DataRepresentationTemplate5_0 {
    pub fn new(
        reference_value: f32,
        binary_scale_factor_e: i16,
        decimal_scale_factor_d: i16,
        number_of_bits: u8,
        original_field_type: OriginalFieldType
    ) -> DataRepresentationTemplate5_0 {
        return DataRepresentationTemplate5_0 {
            reference_value,
            binary_scale_factor_e,
            decimal_scale_factor_d,
            number_of_bits,
            original_field_type
        }
    }
}
