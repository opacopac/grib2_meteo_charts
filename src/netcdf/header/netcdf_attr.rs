use derive_new::new;

use crate::netcdf::header::netcdf_attr_type::NetCdfAttrType;

#[derive(new)]
pub struct NetCdfAttr {
    pub name: String,
    pub nc_type: NetCdfAttrType,
    pub values: Vec<u8> // TODO
}
