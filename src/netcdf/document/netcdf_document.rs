use derive_new::new;

use crate::netcdf::data::netcdf_data::NetCdfData;
use crate::netcdf::header::netcdf_header::NetCdfHeader;

#[derive(new)]
pub struct NetCdfDocument {
    pub header: NetCdfHeader,
    pub data: NetCdfData
}
