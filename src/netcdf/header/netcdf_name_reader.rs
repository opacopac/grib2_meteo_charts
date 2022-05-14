use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::string_reader::StringReader;

pub struct NetCdfNameReader;


impl NetCdfNameReader {
    pub fn read_name<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<String, NetCdfError> {
        let name_len = reader.read_u32::<BigEndian>()?;
        let name = StringReader::read_n_chars(reader, name_len as usize)?;

        let padding = name_len % 4;
        if padding > 0 {
            reader.seek_relative(4 - padding as i64)?;
        }

        return Ok(name);
    }
}
