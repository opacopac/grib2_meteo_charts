use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_values_reader::NetCdfValuesReader;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
use crate::netcdf::header::netcdf_name_reader::NetCdfNameReader;

pub struct NetCdfAttrReader;

impl NetCdfAttrReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfAttr, NetCdfError> {
        let name = NetCdfNameReader::read_name(reader)?;
        let value_type = Self::read_nc_type(reader)?;
        let value_len = reader.read_u32::<BigEndian>()?;
        let values = NetCdfValuesReader::read(reader, value_len, &value_type)?;

        let dim = NetCdfAttr::new(
            name,
            value_type,
            values
        );

        return Ok(dim);
    }


    fn read_nc_type<T: Read>(reader: &mut BufReader<T>) -> Result<NetCdfValueType, NetCdfError> {
        let type_nr = reader.read_u32::<BigEndian>()?;
        let nc_type = match type_nr {
            1 => NetCdfValueType::NcByte,
            2 => NetCdfValueType::NcChar,
            3 => NetCdfValueType::NcShort,
            4 => NetCdfValueType::NcInt,
            5 => NetCdfValueType::NcFloat,
            6 => NetCdfValueType::NcDouble,
            _ => return Err(NetCdfError::InvalidData(
                format!("unknown nc type: {:?}", type_nr)
            ))
        };

        return Ok(nc_type);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcdf_attr_reader::NetCdfAttrReader;
    use crate::netcdf::common::netcdf_value_type::NetCdfValueType;

    #[test]
    fn it_correctly_parses_an_attr_entry() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x05, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x15, 0x49, 0x43, 0x4F, 0x4E, 0x20, 0x67, 0x72, 0x69, 0x64, 0x20, 0x64, 0x65,
            0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00
        ]));

        let result = NetCdfAttrReader::read(&mut reader);
        assert!(result.is_ok());

        let attr = result.unwrap();
        assert_eq!("title", attr.name);
        assert_eq!(NetCdfValueType::NcChar, attr.nc_type);
        //assert_eq!("ICON grid description".len(), attr.values.len());

        assert_eq!(44 as u64, reader.stream_position().unwrap())
    }
}
