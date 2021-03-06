use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_values_reader::NetCdfValuesReader;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::common::netcdf_value_type_reader::NetCdfValueTypeReader;
use crate::netcdf::header::netcdf_name_reader::NetCdfNameReader;

pub struct NetCdfAttrReader;

impl NetCdfAttrReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfAttr, NetCdfError> {
        let name = NetCdfNameReader::read_name(reader)?;
        let value_type = NetCdfValueTypeReader::read(reader)?;
        let value_len = reader.read_u32::<BigEndian>()? as usize;
        let values = NetCdfValuesReader::read(reader, value_len, &value_type)?;

        let dim = NetCdfAttr::new(
            name,
            value_type,
            values
        );

        return Ok(dim);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcdf_attr_reader::NetCdfAttrReader;
    use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
    use crate::netcdf::common::netcdf_values::NetCdfValues;

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

        match attr.values {
            NetCdfValues::CharValues(values) => {
                assert_eq!("ICON grid description".len(), values.len());
                assert_eq!('I', values[0]);
                assert_eq!('n', values[values.len() - 1]);
            },
            _ => panic!("wrong value type: {:?}", attr.values)
        }

        assert_eq!(44 as u64, reader.stream_position().unwrap())
    }
}
