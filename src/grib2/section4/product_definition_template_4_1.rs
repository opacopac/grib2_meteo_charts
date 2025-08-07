use derive_new::new;

use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

#[derive(Debug, new)]
pub struct ProductDefinitionTemplate4_1 {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
}
