use std::io::{BufReader, Read, Seek};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_att_list::NetCdfAttList;

pub struct NetCdfAttListReader;


impl NetCdfAttListReader {
    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfAttList, NetCdfError> {
        let att_list = NetCdfAttList::new(
        );

        return Ok(att_list);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::netcdf::header::netcdf_att_list_reader::NetCdfAttListReader;

    #[test]
    fn it_correctly_parses_the_num_recs() {
        let mut reader = BufReader::new(Cursor::new([
            0x43, 0x44, 0x46, 0x02
        ]));

        let result = NetCdfAttListReader::read(&mut reader);
        assert!(result.is_ok());

        /*let magic = result.unwrap();
        assert_eq!("CDF", magic.magic);
        assert_eq!(2, magic.version);

        assert_eq!(4 as u64, reader.stream_position().unwrap())*/
    }
}
