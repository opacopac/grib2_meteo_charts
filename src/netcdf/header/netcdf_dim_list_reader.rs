use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_dim::NetCdfDim;
use crate::netcdf::header::netcdf_dim_reader::NetCdfDimReader;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufReader, Read, Seek};


pub struct NetCdfDimListReader;


impl NetCdfDimListReader {
    const NC_DIMENSION_TAG: u32 = 0x000A;

    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<Vec<NetCdfDim>, NetCdfError> {
        let nc_dimension = reader.read_u32::<BigEndian>()?;
        let num_elements = reader.read_u32::<BigEndian>()?;

        if nc_dimension == 0 && num_elements == 0 {
            return Ok(vec![]);
        }

        if nc_dimension == Self::NC_DIMENSION_TAG && num_elements > 0 {
            let mut dim_list: Vec<NetCdfDim> = vec![];
            for _ in 0..num_elements {
                let dim = NetCdfDimReader::read(reader)?;
                dim_list.push(dim);
            }
            return Ok(dim_list);
        }

        Err(NetCdfError::InvalidData(
            format!("invalid values for nc_dimensions: '{}' & num_elements: '{}' in dim list!", nc_dimension, num_elements)
        ))
    }
}


#[cfg(test)]
mod tests {
    use crate::netcdf::header::netcdf_dim_list_reader::NetCdfDimListReader;
    use std::io::{BufReader, Cursor, Seek};


    #[test]
    fn it_correctly_parses_the_dim_list() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x04, 0x63, 0x65, 0x6C, 0x6C,
            0x00, 0x2D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x00, 0x00,
            0x00, 0x16, 0x80, 0x02, 0x00, 0x00, 0x00, 0x04, 0x65, 0x64, 0x67, 0x65, 0x00, 0x43, 0x80, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x6E, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02,
            0x6E, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x6E, 0x65, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x02, 0x6E, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x07, 0x74, 0x77, 0x6F, 0x5F, 0x67, 0x72, 0x66, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x09, 0x6D, 0x61, 0x78, 0x5F, 0x63, 0x68, 0x64, 0x6F, 0x6D, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x63, 0x65, 0x6C, 0x6C, 0x5F, 0x67, 0x72, 0x66,
            0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x08, 0x65, 0x64, 0x67, 0x65, 0x5F, 0x67, 0x72, 0x66,
            0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x08, 0x76, 0x65, 0x72, 0x74, 0x5F, 0x67, 0x72, 0x66,
            0x00, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0x0D, 0x63, 0x65, 0x6C, 0x6C, 0x5F, 0x64, 0x65, 0x6C,
            0x61, 0x75, 0x6E, 0x61, 0x79, 0x00, 0x00, 0x00, 0x00, 0x59, 0xFF, 0xFC, 0x00, 0x00, 0x00, 0x0D,
            0x76, 0x65, 0x72, 0x74, 0x5F, 0x64, 0x65, 0x6C, 0x61, 0x75, 0x6E, 0x61, 0x79, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03
        ]));

        let result = NetCdfDimListReader::read(&mut reader);
        //assert!(result.is_ok());

        let dim_list = result.unwrap();
        assert_eq!(14, dim_list.len());
        assert_eq!("cell", dim_list[0].name);
        assert_eq!(2949120, dim_list[0].length);
        assert_eq!("vertex", dim_list[1].name);
        assert_eq!(1474562, dim_list[1].length);
        assert_eq!("edge", dim_list[2].name);
        assert_eq!(4423680, dim_list[2].length);
        assert_eq!("nc", dim_list[3].name);
        assert_eq!(2, dim_list[3].length);
        assert_eq!("nv", dim_list[4].name);
        assert_eq!(3, dim_list[4].length);
        assert_eq!("ne", dim_list[5].name);
        assert_eq!(6, dim_list[5].length);
        assert_eq!("no", dim_list[6].name);
        assert_eq!(4, dim_list[6].length);
        assert_eq!("two_grf", dim_list[7].name);
        assert_eq!(2, dim_list[7].length);
        assert_eq!("max_chdom", dim_list[8].name);
        assert_eq!(1, dim_list[8].length);
        assert_eq!("cell_grf", dim_list[9].name);
        assert_eq!(14, dim_list[9].length);
        assert_eq!("edge_grf", dim_list[10].name);
        assert_eq!(24, dim_list[10].length);
        assert_eq!("vert_grf", dim_list[11].name);
        assert_eq!(13, dim_list[11].length);
        assert_eq!("cell_delaunay", dim_list[12].name);
        assert_eq!(5898236, dim_list[12].length);
        assert_eq!("vert_delaunay", dim_list[13].name);
        assert_eq!(3, dim_list[13].length);

        assert_eq!(228 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_an_absent_dim_list() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ]));

        let result = NetCdfDimListReader::read(&mut reader);
        assert!(result.is_ok());

        let dim_list = result.unwrap();
        assert_eq!(0, dim_list.len());

        assert_eq!(8 as u64, reader.stream_position().unwrap())
    }
}
