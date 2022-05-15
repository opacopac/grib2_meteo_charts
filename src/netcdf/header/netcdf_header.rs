use std::fmt::{Display, Formatter};

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


impl Display for NetCdfHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HEADER:")?;
        writeln!(f, "  magic:\n{}", self.magic)?;
        writeln!(f, "  num_recs: {}", self.num_recs)?;

        writeln!(f, "  dim_list:")?;
        self.dim_list.iter().for_each(|dim| write!(f, "{}", dim).unwrap());
        writeln!(f)?;

        writeln!(f, "  att_list:")?;
        self.att_list.iter().for_each(|att| write!(f, "{}", att).unwrap());
        writeln!(f)?;

        writeln!(f, "  var_list:")?;
        self.var_list.iter().for_each(|var| write!(f, "{}", var).unwrap());
        writeln!(f)?;

        return Ok(());
    }
}
