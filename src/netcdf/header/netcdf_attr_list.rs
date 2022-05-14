use derive_new::new;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;

#[derive(new)]
pub struct NetCdfAttrList {
    pub(crate) attributes: Vec<NetCdfAttr>
}
