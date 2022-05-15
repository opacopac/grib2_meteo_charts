use std::fmt::{Display, Formatter};
use derive_new::new;

use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
use crate::netcdf::common::netcdf_values::NetCdfValues;

#[derive(new)]
pub struct NetCdfAttr {
    pub name: String,
    pub nc_type: NetCdfValueType,
    pub values: NetCdfValues
}


impl Display for NetCdfAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ATTR:")?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  type: {}", self.nc_type)?;
        writeln!(f, "  values: {}", self.values)?;

        return Ok(());
    }
}
