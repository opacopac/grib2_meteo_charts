use std::fmt::{Display, Formatter};

use derive_new::new;

#[derive(new)]
pub struct NetCdfDim {
    pub name: String,
    pub length: u32
}


impl Display for NetCdfDim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DIM:")?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  length: {}", self.length)?;

        return Ok(());
    }
}
