use std::fs::File;
use std::io::{BufReader, Read, Seek};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::data::netcdf_data_reader::NetCdfDataReader;
use crate::netcdf::document::netcdf_document::NetCdfDocument;
use crate::netcdf::header::netcdf_header_reader::NetCdfHeaderReader;

pub struct NetCdfDocumentReader;


impl NetCdfDocumentReader {
    pub fn read_file(filename: &str, var_names: Vec<&str>) -> Result<NetCdfDocument, NetCdfError> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let doc = Self::read(&mut reader, var_names)?;

        return Ok(doc);
    }


    pub fn read<T: Read + Seek>(reader: &mut BufReader<T>, var_names: Vec<&str>) -> Result<NetCdfDocument, NetCdfError> {
        let header = NetCdfHeaderReader::read(reader)?;
        let data_map = NetCdfDataReader::read_data_map(reader, &header, var_names)?;

        let doc = NetCdfDocument::new(
            header,
            data_map
        );

        return Ok(doc);
    }
}
