use std::collections::HashMap;

use derive_new::new;

use crate::netcdf::common::netcdf_values::NetCdfValues;
use crate::netcdf::header::netcdf_header::NetCdfHeader;

#[derive(new)]
pub struct NetCdfDocument {
    pub header: NetCdfHeader,
    pub data_map: HashMap<String, NetCdfValues>
}
