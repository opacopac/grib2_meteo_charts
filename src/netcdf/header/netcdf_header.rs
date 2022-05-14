use derive_new::new;

use crate::netcdf::header::netcdf_magic::NetCdfMagic;

#[derive(new)]
pub struct NetCdfHeader {
    pub magic: NetCdfMagic,
    pub num_recs: u32
}
