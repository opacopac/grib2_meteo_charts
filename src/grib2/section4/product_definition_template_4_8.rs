use derive_new::new;

use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

#[derive(Debug, new)]
pub struct ProductDefinitionTemplate4_8 {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
}

impl ProductDefinitionTemplate4_8 {
    pub const TPL_LENGTH_BYTES: u32 = 2;
}
