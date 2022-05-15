use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum NetCdfValueType {
    NcByte,
    NcChar,
    NcShort,
    NcInt,
    NcFloat,
    NcDouble
}

impl Display for NetCdfValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetCdfValueType::NcByte => write!(f, "Byte"),
            NetCdfValueType::NcChar => write!(f, "Char"),
            NetCdfValueType::NcShort => write!(f, "Short"),
            NetCdfValueType::NcInt => write!(f, "Int"),
            NetCdfValueType::NcFloat => write!(f, "Float"),
            NetCdfValueType::NcDouble => write!(f, "Double"),
        }
    }
}
