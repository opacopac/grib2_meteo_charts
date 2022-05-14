use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::string_reader::StringReader;
use crate::netcdf::header::netcdf_dim::NetCdfDim;

pub struct NetCdfDimReader;

impl NetCdfDimReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfDim, NetCdfError> {
        let name_len = reader.read_u32::<BigEndian>()?;
        let name = StringReader::read_n_chars(reader, name_len as usize)?;

        let padding = name_len % 4;
        if padding > 0 {
            reader.seek_relative(4 - padding as i64)?;
        }

        let dim_len = reader.read_u32::<BigEndian>()?;

        let dim = NetCdfDim::new(
            name,
            dim_len
        );

        return Ok(dim);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcdf_dim_reader::NetCdfDimReader;

    #[test]
    fn it_correctly_parses_a_dim_entry_without_padding() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x04, 0x63, 0x65, 0x6C, 0x6C, 0x00, 0x2D, 0x00, 0x00
        ]));

        let result = NetCdfDimReader::read(&mut reader);
        assert!(result.is_ok());

        let dim = result.unwrap();
        assert_eq!("cell", dim.name);
        assert_eq!(0x002D0000, dim.length);

        assert_eq!(12 as u64, reader.stream_position().unwrap())
    }

    #[test]
    fn it_correctly_parses_a_dim_entry_with_padding() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x06, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x00, 0x00, 0x00, 0x16, 0x80, 0x02
        ]));

        let result = NetCdfDimReader::read(&mut reader);
        assert!(result.is_ok());

        let dim = result.unwrap();
        assert_eq!("vertex", dim.name);
        assert_eq!(0x00168002, dim.length);

        assert_eq!(16 as u64, reader.stream_position().unwrap())
    }
}
