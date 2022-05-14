use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_name_reader::NetCdfNameReader;
use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::header::netcdf_attr_type::NetCdfAttrType;

pub struct NetCdfAttrReader;

impl NetCdfAttrReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfAttr, NetCdfError> {
        let name = NetCdfNameReader::read_name(reader)?;
        let nc_type = Self::read_nc_type(reader)?;
        let nc_values = Self::read_nc_values(reader)?;

        let dim = NetCdfAttr::new(
            name,
            nc_type,
            nc_values
        );

        return Ok(dim);
    }


    fn read_nc_type<T: Read>(reader: &mut BufReader<T>) -> Result<NetCdfAttrType, NetCdfError> {
        let type_nr = reader.read_u32::<BigEndian>()?;
        let nc_type = match type_nr {
            1 => NetCdfAttrType::NcByte,
            2 => NetCdfAttrType::NcChar,
            3 => NetCdfAttrType::NcShort,
            4 => NetCdfAttrType::NcInt,
            5 => NetCdfAttrType::NcFloat,
            6 => NetCdfAttrType::NcDouble,
            _ => return Err(NetCdfError::InvalidData(
                format!("unknown nc type: {:?}", type_nr)
            ))
        };

        return Ok(nc_type);
    }


    fn read_nc_values<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Vec<u8>, NetCdfError> {
        let value_len = reader.read_u32::<BigEndian>()?;
        let mut values: Vec<u8> = vec![];
        for _ in 0..value_len {
            let value = reader.read_u8()?;
            values.push(value);
        }

        let padding = value_len % 4;
        if padding > 0 {
            reader.seek_relative(4 - padding as i64)?;
        }

        // TODO: >u16/u32 types

        return Ok(values);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcdf_attr_reader::NetCdfAttrReader;
    use crate::netcdf::header::netcdf_attr_type::NetCdfAttrType;

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
        assert_eq!(NetCdfAttrType::NcChar, attr.nc_type);
        assert_eq!("ICON grid description".len(), attr.values.len());

        assert_eq!(44 as u64, reader.stream_position().unwrap())
    }
}
