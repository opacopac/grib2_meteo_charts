use std::io::{BufReader, Cursor, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_attr_list_reader::NetCdfAttrListReader;
use crate::netcdf::header::netcdf_dim_list_reader::NetCdfDimListReader;
use crate::netcdf::header::netcdf_header::NetCdfHeader;
use crate::netcdf::header::netcdf_magic_reader::NetCdfMagicReader;
use crate::netcdf::header::netcdf_var_list_reader::NetCdfVarListReader;

pub struct NetCdfHeaderReader;


impl NetCdfHeaderReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfHeader, NetCdfError> {
        let magic = NetCdfMagicReader::read(reader)?;
        let num_recs = reader.read_u32::<BigEndian>()?;
        let dim_list = NetCdfDimListReader::read(reader)?;
        let att_list = NetCdfAttrListReader::read(reader)?;
        let var_list = NetCdfVarListReader::read(reader)?;

        let header = NetCdfHeader::new(
            magic,
            num_recs,
            dim_list,
            att_list,
            var_list
        );

        return Ok(header);
    }
}


#[test]
fn it_correctly_parses_a_header_of_an_file_with_empty_lists() {
    let mut reader = BufReader::new(Cursor::new([
        0x43, 0x44, 0x46, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00
    ]));

    let result = NetCdfHeaderReader::read(&mut reader);
    assert!(result.is_ok());

    let header = result.unwrap();
    assert_eq!("CDF".to_string(), header.magic.magic);
    assert_eq!(2, header.magic.version);
    assert_eq!(0, header.dim_list.dims.len());
    assert_eq!(0, header.att_list.len());
    assert_eq!(0, header.var_list.len());

    assert_eq!(20 as u64, reader.stream_position().unwrap())
}
