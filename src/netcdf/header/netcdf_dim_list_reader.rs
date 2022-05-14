use std::io::{BufReader, Read, Seek};
use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_dim_list::NetCdfDimList;

pub struct NetCdfDimListReader;

impl NetCdfDimListReader {
    const NC_DIMENSION: u32 = 0x000A;

    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfDimList, NetCdfError> {
        let nc_dimension = reader.read_u32::<BigEndian>()?;
        if nc_dimension != Self::NC_DIMENSION {
            let empty_dim_list = NetCdfDimList::new(0);
            return Ok(empty_dim_list);
        }

        let num_elements = reader.read_u32::<BigEndian>()?;

        let dim_list = NetCdfDimList::new(
            num_elements
        );

        return Ok(dim_list);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::header::netcdf_dim_list_reader::NetCdfDimListReader;

    #[test]
    fn it_correctly_parses_the_dim_list() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x0E
        ]));

        let result = NetCdfDimListReader::read(&mut reader);
        assert!(result.is_ok());

        let dim_list = result.unwrap();
        assert_eq!(14, dim_list.num_elements);

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_an_absent_dim_list() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x00
        ]));

        let result = NetCdfDimListReader::read(&mut reader);
        assert!(result.is_ok());

        let dim_list = result.unwrap();
        assert_eq!(0, dim_list.num_elements);

        assert_eq!(4 as u64, reader.stream_position().unwrap())
    }
}
