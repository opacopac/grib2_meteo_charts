use std::fmt::{Display, Formatter};

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


impl Display for NetCdfVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VAR:")?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  nc_type: {}", self.nc_type)?;
        writeln!(f, "  size: {}", self.size)?;
        writeln!(f, "  begin: {}", self.begin)?;

        write!(f, "  dim_ids: ")?;
        self.dim_ids.iter().for_each(|dim_id| write!(f, "{} ", dim_id).unwrap());
        writeln!(f)?;

        writeln!(f, "  attributes: {}", self.attributes.len())?;
        self.attributes.iter().for_each(|attr| write!(f, "{}", attr).unwrap());
        writeln!(f)?;

        return Ok(());
    }
}
