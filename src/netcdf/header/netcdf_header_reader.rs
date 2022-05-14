use std::io::{BufReader, Read, Seek};
use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_header::NetCdfHeader;
use crate::netcdf::header::netcdf_magic_reader::NetCdfMagicReader;

pub struct NetCdfHeaderReader;


impl NetCdfHeaderReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfHeader, NetCdfError> {
        let magic = NetCdfMagicReader::read(reader)?;
        let num_recs = reader.read_u32::<BigEndian>()?;

        let header = NetCdfHeader::new(
            magic,
            num_recs
        );

        return Ok(header);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::netcdf::header::netcdf_header_reader::NetCdfHeaderReader;

    #[test]
    fn it_correctly_parses_the_header() {
        let mut reader = BufReader::new(Cursor::new([
            0x43, 0x44, 0x46, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x0E,
            0x00, 0x00, 0x00, 0x04, 0x63, 0x65, 0x6C, 0x6C, 0x00, 0x2D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06
        ]));

        let result = NetCdfHeaderReader::read(&mut reader);
        assert!(result.is_ok());

        let header = result.unwrap();
        assert_eq!("CDF", header.magic.magic);
        assert_eq!(2, header.magic.version);
        assert_eq!(0, header.num_recs);
    }
}
