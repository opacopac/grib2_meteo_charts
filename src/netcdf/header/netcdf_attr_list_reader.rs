use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::header::netcdf_attr::NetCdfAttr;
use crate::netcdf::header::netcdf_attr_reader::NetCdfAttrReader;

pub struct NetCdfAttrListReader;


impl NetCdfAttrListReader {
    const NC_ATTRIBUTE_TAG: u32 = 0x000C;

    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<Vec<NetCdfAttr>, NetCdfError> {
        let nc_attribute_tag = reader.read_u32::<BigEndian>()?;
        if nc_attribute_tag != Self::NC_ATTRIBUTE_TAG {
            return Ok(vec![]);
        }

        let mut attributes: Vec<NetCdfAttr> = vec![];
        let num_elements = reader.read_u32::<BigEndian>()?;
        for _ in 0..num_elements {
            let attr = NetCdfAttrReader::read(reader)?;
            attributes.push(attr);
        }

        return Ok(attributes);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::netcdf::common::netcdf_value_type::NetCdfValueType;
    use crate::netcdf::header::netcdf_attr_list_reader::NetCdfAttrListReader;

    #[test]
    fn it_correctly_parses_the_num_recs() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x05, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x15, 0x49, 0x43, 0x4F, 0x4E,
            0x20, 0x67, 0x72, 0x69, 0x64, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x68, 0x69, 0x73, 0x74, 0x6F, 0x72, 0x79, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x4E, 0x2F, 0x70, 0x61, 0x6E, 0x66, 0x73, 0x2F, 0x65, 0x2F, 0x76, 0x6F, 0x6C, 0x32, 0x2F, 0x67, 0x7A, 0x61, 0x65, 0x6E, 0x67, 0x6C, 0x2F, 0x69, 0x63,
            0x6F, 0x6E, 0x2D, 0x64, 0x65, 0x76, 0x2F, 0x62, 0x75, 0x69, 0x6C, 0x64, 0x2F, 0x78, 0x38, 0x36, 0x5F, 0x36, 0x34, 0x2D, 0x75, 0x6E, 0x6B, 0x6E, 0x6F, 0x77, 0x6E, 0x2D, 0x6C, 0x69, 0x6E, 0x75,
            0x78, 0x2D, 0x67, 0x6E, 0x75, 0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x63, 0x6F, 0x6D, 0x6D, 0x61, 0x6E, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x69, 0x6E, 0x73, 0x74,
            0x69, 0x74, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x3B, 0x4D, 0x61, 0x78, 0x20, 0x50, 0x6C, 0x61, 0x6E, 0x63, 0x6B, 0x20, 0x49, 0x6E, 0x73, 0x74, 0x69,
            0x74, 0x75, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x4D, 0x65, 0x74, 0x65, 0x6F, 0x72, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x2F, 0x44, 0x65, 0x75, 0x74, 0x73, 0x63, 0x68, 0x65, 0x72, 0x20, 0x57,
            0x65, 0x74, 0x74, 0x65, 0x72, 0x64, 0x69, 0x65, 0x6E, 0x73, 0x74, 0x00, 0x00, 0x00, 0x00, 0x06, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A,
            0x69, 0x63, 0x6F, 0x6E, 0x2D, 0x64, 0x65, 0x76, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x75, 0x75, 0x69, 0x64, 0x4F, 0x66, 0x48, 0x47, 0x72, 0x69, 0x64, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x24, 0x61, 0x32, 0x37, 0x62, 0x38, 0x64, 0x65, 0x36, 0x2D, 0x31, 0x38, 0x63, 0x34, 0x2D, 0x31, 0x31, 0x65, 0x34, 0x2D, 0x38, 0x32, 0x30, 0x61, 0x2D, 0x62, 0x35, 0x62, 0x30,
            0x39, 0x38, 0x63, 0x36, 0x61, 0x35, 0x63, 0x30, 0x00, 0x00, 0x00, 0x13, 0x6E, 0x75, 0x6D, 0x62, 0x65, 0x72, 0x5F, 0x6F, 0x66, 0x5F, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x75, 0x73, 0x65, 0x64, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x00, 0x00, 0x12, 0x49, 0x43, 0x4F, 0x4E, 0x5F, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x66, 0x69, 0x6C, 0x65, 0x5F, 0x75,
            0x72, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x4B, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x69, 0x63, 0x6F, 0x6E, 0x2D, 0x64, 0x6F, 0x77, 0x6E, 0x6C, 0x6F, 0x61, 0x64,
            0x73, 0x2E, 0x6D, 0x70, 0x69, 0x6D, 0x65, 0x74, 0x2E, 0x6D, 0x70, 0x67, 0x2E, 0x64, 0x65, 0x2F, 0x67, 0x72, 0x69, 0x64, 0x73, 0x2F, 0x70, 0x75, 0x62, 0x6C, 0x69, 0x63, 0x2F, 0x69, 0x63, 0x6F,
            0x6E, 0x5F, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x30, 0x30, 0x32, 0x36, 0x5F, 0x52, 0x30, 0x33, 0x42, 0x30, 0x37, 0x5F, 0x47, 0x2E, 0x6E, 0x63, 0x00, 0x00, 0x00, 0x00, 0x06, 0x63, 0x65, 0x6E, 0x74,
            0x72, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4E, 0x00, 0x00, 0x00, 0x09, 0x73, 0x75, 0x62, 0x63, 0x65, 0x6E, 0x74, 0x72, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x0D, 0x6F, 0x75, 0x74, 0x6E, 0x61, 0x6D, 0x65, 0x5F, 0x73, 0x74, 0x79, 0x6C, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x11, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x6D, 0x61, 0x70, 0x70, 0x69, 0x6E, 0x67, 0x5F, 0x6E, 0x61, 0x6D,
            0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x12, 0x6C, 0x61, 0x74, 0x5F, 0x6C, 0x6F, 0x6E, 0x67, 0x5F, 0x6F, 0x6E, 0x5F, 0x73, 0x70, 0x68, 0x65, 0x72, 0x65, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x06, 0x63, 0x72, 0x73, 0x5F, 0x69, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x1C, 0x75, 0x72, 0x6E, 0x3A, 0x6F, 0x67, 0x63, 0x3A, 0x64, 0x65, 0x66, 0x3A,
            0x63, 0x73, 0x3A, 0x45, 0x50, 0x53, 0x47, 0x3A, 0x36, 0x2E, 0x30, 0x3A, 0x36, 0x34, 0x32, 0x32, 0x00, 0x00, 0x00, 0x08, 0x63, 0x72, 0x73, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x1E, 0x53, 0x70, 0x68, 0x65, 0x72, 0x69, 0x63, 0x61, 0x6C, 0x20, 0x32, 0x44, 0x20, 0x43, 0x6F, 0x6F, 0x72, 0x64, 0x69, 0x6E, 0x61, 0x74, 0x65, 0x20, 0x53, 0x79, 0x73, 0x74,
            0x65, 0x6D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x65, 0x6C, 0x6C, 0x69, 0x70, 0x73, 0x6F, 0x69, 0x64, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06,
            0x53, 0x70, 0x68, 0x65, 0x72, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0F, 0x73, 0x65, 0x6D, 0x69, 0x5F, 0x6D, 0x61, 0x6A, 0x6F, 0x72, 0x5F, 0x61, 0x78, 0x69, 0x73, 0x00, 0x00, 0x00, 0x00, 0x06,
            0x00, 0x00, 0x00, 0x01, 0x41, 0x58, 0x4D, 0xE7, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x69, 0x6E, 0x76, 0x65, 0x72, 0x73, 0x65, 0x5F, 0x66, 0x6C, 0x61, 0x74, 0x74, 0x65, 0x6E, 0x69,
            0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x6C, 0x65, 0x76,
            0x65, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x09, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x72, 0x6F, 0x6F, 0x74, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x07, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x49, 0x44, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0E, 0x70, 0x61, 0x72, 0x65, 0x6E, 0x74, 0x5F, 0x67, 0x72, 0x69, 0x64, 0x5F, 0x49, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x6D, 0x61, 0x78, 0x5F, 0x63, 0x68, 0x69, 0x6C, 0x64, 0x64, 0x6F, 0x6D, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01
        ]));

        let result = NetCdfAttrListReader::read(&mut reader);
        assert!(result.is_ok());

        let attr_list = result.unwrap();
        assert_eq!(21, attr_list.len());

        assert_eq!("title", attr_list[0].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[0].nc_type);
        assert_eq!("ICON grid description", attr_list[0].values.get_chars());

        assert_eq!("history", attr_list[1].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[1].nc_type);
        assert_eq!("/panfs/e/vol2/gzaengl/icon-dev/build/x86_64-unknown-linux-gnu/bin/grid_command", attr_list[1].values.get_chars());

        assert_eq!("institution", attr_list[2].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[2].nc_type);
        assert_eq!("Max Planck Institute for Meteorology/Deutscher Wetterdienst", attr_list[2].values.get_chars());

        assert_eq!("source", attr_list[3].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[3].nc_type);
        // assert_eq!("icon-dev", get_chars(&attr_list.attributes[3].values));

        assert_eq!("uuidOfHGrid", attr_list[4].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[4].nc_type);
        assert_eq!("a27b8de6-18c4-11e4-820a-b5b098c6a5c0", attr_list[4].values.get_chars());

        assert_eq!("number_of_grid_used", attr_list[5].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[5].nc_type);
        assert_eq!(vec![26], attr_list[5].values.get_ints());

        assert_eq!("ICON_grid_file_uri", attr_list[6].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[6].nc_type);
        assert_eq!("http://icon-downloads.mpimet.mpg.de/grids/public/icon_grid_0026_R03B07_G.nc", attr_list[6].values.get_chars());

        assert_eq!("centre", attr_list[7].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[7].nc_type);
        assert_eq!(vec![78], attr_list[7].values.get_ints());

        assert_eq!("subcentre", attr_list[8].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[8].nc_type);
        assert_eq!(vec![255], attr_list[8].values.get_ints());

        assert_eq!("outname_style", attr_list[9].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[9].nc_type);
        assert_eq!(vec![2], attr_list[9].values.get_ints());

        assert_eq!("grid_mapping_name", attr_list[10].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[10].nc_type);
        assert_eq!("lat_long_on_sphere", attr_list[10].values.get_chars());

        assert_eq!("crs_id", attr_list[11].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[11].nc_type);
        assert_eq!("urn:ogc:def:cs:EPSG:6.0:6422", attr_list[11].values.get_chars());

        assert_eq!("crs_name", attr_list[12].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[12].nc_type);
        assert_eq!("Spherical 2D Coordinate System", attr_list[12].values.get_chars());

        assert_eq!("ellipsoid_name", attr_list[13].name);
        assert_eq!(NetCdfValueType::NcChar, attr_list[13].nc_type);
        assert_eq!("Sphere", attr_list[13].values.get_chars());

        assert_eq!("semi_major_axis", attr_list[14].name);
        assert_eq!(NetCdfValueType::NcDouble, attr_list[14].nc_type);
        assert_eq!(vec![6371229.0], attr_list[14].values.get_doubles());

        assert_eq!("inverse_flattening", attr_list[15].name);
        assert_eq!(NetCdfValueType::NcDouble, attr_list[15].nc_type);
        assert_eq!(vec![0.0], attr_list[15].values.get_doubles());

        assert_eq!("grid_level", attr_list[16].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[16].nc_type);
        assert_eq!(vec![7], attr_list[16].values.get_ints());

        assert_eq!("grid_root", attr_list[17].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[17].nc_type);
        assert_eq!(vec![3], attr_list[17].values.get_ints());

        assert_eq!("grid_ID", attr_list[18].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[18].nc_type);
        assert_eq!(vec![1], attr_list[18].values.get_ints());

        assert_eq!("parent_grid_ID", attr_list[19].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[19].nc_type);
        assert_eq!(vec![0], attr_list[19].values.get_ints());

        assert_eq!("max_childdom", attr_list[20].name);
        assert_eq!(NetCdfValueType::NcInt, attr_list[20].nc_type);
        assert_eq!(vec![1], attr_list[20].values.get_ints());

        assert_eq!(960 as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_an_absent_attr_list() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x00
        ]));

        let result = NetCdfAttrListReader::read(&mut reader);
        assert!(result.is_ok());

        let attr_list = result.unwrap();
        assert_eq!(0, attr_list.len());

        assert_eq!(4 as u64, reader.stream_position().unwrap())
    }
}
