use std::io::{BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_value_type::NetCdfValueType;

pub struct NetCdfValueTypeReader;


impl NetCdfValueTypeReader {
    pub fn read<T: Read>(reader: &mut BufReader<T>) -> Result<NetCdfValueType, NetCdfError> {
        let type_nr = reader.read_u32::<BigEndian>()?;
        let nc_type = match type_nr {
            1 => NetCdfValueType::NcByte,
            2 => NetCdfValueType::NcChar,
            3 => NetCdfValueType::NcShort,
            4 => NetCdfValueType::NcInt,
            5 => NetCdfValueType::NcFloat,
            6 => NetCdfValueType::NcDouble,
            _ => panic!("unknown nc type: {:?}", type_nr) /*return Err(NetCdfError::InvalidData(
                format!("unknown nc type: {:?}", type_nr)
            ))*/
        };

        return Ok(nc_type);
    }
}
