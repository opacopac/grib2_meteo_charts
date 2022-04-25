use derive_more::Constructor;

use crate::grib2::common::scale_factor_value::ScaleFactorValue;

#[derive(Debug, Constructor)]
pub struct ProductDefinitionTemplate4_0 {
    pub parameter_category: u8,
    pub parameter_number: u8,
    pub generating_process_type: u8,
    pub generating_process_identifier: u8,
    pub generating_process: u8,
    pub hours_cutoff: u16,
    pub mins_cutoff: u8,
    pub forecast_time_unit: u8,
    pub forecast_time_value: u8,
    pub fixed_surface1_type: u8,
    pub fixed_surface1_scale_factor_value: ScaleFactorValue,
    pub fixed_surface2_type: u8,
    pub fixed_surface2_scale_factor_value: ScaleFactorValue,
}

impl ProductDefinitionTemplate4_0 {
    pub const TPL_LENGTH_BYTES: u32 = 22;
}
