use derive_new::new;

use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::header::netcdf_dim::NetCdfDim;
use crate::netcdf::header::netcdf_magic::NetCdfMagic;
use crate::netcdf::header::netcdf_var::NetCdfVar;

#[derive(new)]
pub struct NetCdfHeader {
    pub magic: NetCdfMagic,
    pub num_recs: u32,
    pub dim_list: Vec<NetCdfDim>,
    pub att_list: Vec<NetCdfAttr>,
    pub var_list: Vec<NetCdfVar>
}
