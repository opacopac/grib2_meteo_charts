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


impl NetCdfValueType {
    pub fn get_byte_size(&self) -> u8 {
        return match self {
            NetCdfValueType::NcByte => 1,
            NetCdfValueType::NcChar => 1,
            NetCdfValueType::NcShort => 2,
            NetCdfValueType::NcInt => 4,
            NetCdfValueType::NcFloat => 4,
            NetCdfValueType::NcDouble => 8,
        };
    }
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
