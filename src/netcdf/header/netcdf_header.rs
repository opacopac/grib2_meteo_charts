use derive_new::new;

use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::header::netcdf_dim_list::NetCdfDimList;
use crate::netcdf::header::netcdf_magic::NetCdfMagic;
use crate::netcdf::header::netcdf_var::NetCdfVar;

#[derive(new)]
pub struct NetCdfHeader {
    pub magic: NetCdfMagic,
    pub num_recs: u32,
    pub dim_list: NetCdfDimList,
    pub att_list: Vec<NetCdfAttr>,
    pub var_list: Vec<NetCdfVar>
}
