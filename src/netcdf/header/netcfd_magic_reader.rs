use std::io::{BufReader, Read, Seek};

use byteorder::ReadBytesExt;

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::string_reader::StringReader;
use crate::netcdf::header::netcfd_magic::NetCdfMagic;

pub struct NetCdfMagicReader;


impl NetCdfMagicReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfMagic, NetCdfError> {
        let magic_value = StringReader::read_n_chars(reader, 3)?;
        let version = reader.read_u8()?;

        let magic = NetCdfMagic::new(
            magic_value,
            version
        )?;

        return Ok(magic);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcfd_magic_reader::NetCdfMagicReader;

    #[test]
    fn it_correctly_parses_the_magic() {
        let mut reader = BufReader::new(Cursor::new([
            0x43, 0x44, 0x46, 0x02
        ]));

        let result = NetCdfMagicReader::read(&mut reader);
        assert!(result.is_ok());

        let magic = result.unwrap();
        assert_eq!("CDF", magic.magic);
        assert_eq!(2, magic.version);

        assert_eq!(4 as u64, reader.stream_position().unwrap())
    }
}
