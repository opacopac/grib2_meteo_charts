use derive_new::new;

use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

#[derive(Debug, new)]
pub struct ProductDefinitionTemplate4_1 {
    pub parameter_category: MeteoParameterCategory,
    pub parameter_number: u8,
    pub type_of_generating_process: u8,
    pub background_generating_process_identifier: u8,
    pub forecast_generating_process_identifier: u8,
    pub hours_after_reference_time: u16,
    pub minutes_after_reference_time: u8,
    pub time_range_unit: u8,
    pub forecast_time_in_units_defined_by_octet_18: u32,
    pub type_of_first_fixed_surface: u8,
    pub scale_factor_of_first_fixed_surface: i8,
    pub scaled_value_of_first_fixed_surface: u32,
    pub type_of_second_fixed_surface: u8,
    pub scale_factor_of_second_fixed_surface: i8,
    pub scaled_value_of_second_fixed_surface: u32,
    pub type_of_ensemble_forecast: u8,
    pub perturbation_number: u8,
    pub number_of_forecasts_in_ensemble: u8,
}
