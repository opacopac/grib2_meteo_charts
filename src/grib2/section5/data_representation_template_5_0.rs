use crate::grib2::section5::original_field_type::OriginalFieldType;

pub struct DataRepresentationTemplate5_0 {
    pub reference_value: u32,
    pub binary_scale_factor: u16,
    pub decimal_scale_factor: u16,
    pub number_of_bits: u8,
    pub original_field_type: OriginalFieldType
}


impl DataRepresentationTemplate5_0 {
    pub fn new(
        reference_value: u32,
        binary_scale_factor: u16,
        decimal_scale_factor: u16,
        number_of_bits: u8,
        original_field_type: OriginalFieldType
    ) -> DataRepresentationTemplate5_0 {
        return DataRepresentationTemplate5_0 {
            reference_value,
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits,
            original_field_type
        }
    }
}
