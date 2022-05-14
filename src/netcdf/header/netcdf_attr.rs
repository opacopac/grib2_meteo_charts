use derive_new::new;

use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
use crate::netcdf::common::netcdf_values::NetCdfValues;

#[derive(new)]
pub struct NetCdfAttr {
    pub name: String,
    pub nc_type: NetCdfValueType,
    pub values: NetCdfValues
}
