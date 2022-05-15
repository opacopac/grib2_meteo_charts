use derive_new::new;

use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;

#[derive(new)]
pub struct NetCdfVar {
    pub name: String,
    pub dim_ids: Vec<u32>,
    pub attributes: Vec<NetCdfAttr>,
    pub nc_type: NetCdfValueType,
    pub size: u32,
    pub begin: u64
}
