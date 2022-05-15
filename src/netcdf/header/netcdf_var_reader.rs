use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_value_type_reader::NetCdfValueTypeReader;
use crate::netcdf::header::netcdf_attr_list_reader::NetCdfAttrListReader;
use crate::netcdf::header::netcdf_name_reader::NetCdfNameReader;
use crate::netcdf::header::netcdf_var::NetCdfVar;

pub struct NetCdfVarReader;

impl NetCdfVarReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfVar, NetCdfError> {
        let name = NetCdfNameReader::read_name(reader)?;
        let dim_id_len = reader.read_u32::<BigEndian>()?;
        let mut dim_ids: Vec<u32> = vec![];
        for _ in 0..dim_id_len {
            let dim_id = reader.read_u32::<BigEndian>()?;
            dim_ids.push(dim_id);
        }
        let attributes = NetCdfAttrListReader::read(reader)?;
        let nc_type = NetCdfValueTypeReader::read(reader)?;
        let var_size = reader.read_u32::<BigEndian>()?;
        let begin = reader.read_u64::<BigEndian>()?;

        let var = NetCdfVar::new(
            name,
            dim_ids,
            attributes,
            nc_type,
            var_size,
            begin
        );

        return Ok(var);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
    use crate::netcdf::header::netcdf_var_reader::NetCdfVarReader;

    #[test]
    fn it_correctly_parses_a_var_entry() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x04, 0x63, 0x6C, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x09, 0x6C, 0x6F, 0x6E, 0x67,
            0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10,
            0x63, 0x65, 0x6E, 0x74, 0x65, 0x72, 0x20, 0x6C, 0x6F, 0x6E, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
            0x00, 0x00, 0x00, 0x05, 0x75, 0x6E, 0x69, 0x74, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x06, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D,
            0x73, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0E, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x6C, 0x6F, 0x6E,
            0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x62, 0x6F, 0x75, 0x6E,
            0x64, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0D, 0x63, 0x6C, 0x6F, 0x6E,
            0x5F, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x65, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
            0x01, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x28
        ]));

        let result = NetCdfVarReader::read(&mut reader);
        assert!(result.is_ok());

        let var = result.unwrap();
        assert_eq!("clon", var.name);
        assert_eq!(1, var.dim_ids.len());
        assert_eq!(0, var.dim_ids[0]);
        assert_eq!(4, var.attributes.len());
        assert_eq!("long_name", var.attributes[0].name);
        assert_eq!(NetCdfValueType::NcChar, var.attributes[0].nc_type);
        assert_eq!("center longitude", var.attributes[0].values.get_chars());
        assert_eq!("units", var.attributes[1].name);
        assert_eq!(NetCdfValueType::NcChar, var.attributes[1].nc_type);
        assert_eq!("radian", var.attributes[1].values.get_chars());
        assert_eq!("standard_name", var.attributes[2].name);
        assert_eq!(NetCdfValueType::NcChar, var.attributes[2].nc_type);
        assert_eq!("grid_longitude", var.attributes[2].values.get_chars());
        assert_eq!("bounds", var.attributes[3].name);
        assert_eq!(NetCdfValueType::NcChar, var.attributes[3].nc_type);
        assert_eq!("clon_vertices", var.attributes[3].values.get_chars());
        assert_eq!(NetCdfValueType::NcDouble, var.nc_type);
        assert_eq!(23592960, var.size);
        assert_eq!(8488, var.begin);

        assert_eq!(188 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_a_var_entry_without_attributes() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x0B, 0x63, 0x63, 0x5F, 0x64, 0x65, 0x6C, 0x61, 0x75, 0x6E, 0x61, 0x79, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x01, 0xDF, 0xD0, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x35, 0xF1, 0x84
        ]));

        let result = NetCdfVarReader::read(&mut reader);
        assert!(result.is_ok());

        let var = result.unwrap();
        assert_eq!("cc_delaunay", var.name);
        assert_eq!(2, var.dim_ids.len());
        assert_eq!(13, var.dim_ids[0]);
        assert_eq!(12, var.dim_ids[1]);
        assert_eq!(0, var.attributes.len());
        assert_eq!(122832, var.size);
        assert_eq!(3535236, var.begin);

        assert_eq!(52 as u64, reader.stream_position().unwrap())
    }
}
