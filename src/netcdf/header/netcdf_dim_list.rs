use derive_new::new;

use crate::netcdf::header::netcdf_dim::NetCdfDim;

#[derive(new)]
pub struct NetCdfDimList {
    pub dims: Vec<NetCdfDim>
}
