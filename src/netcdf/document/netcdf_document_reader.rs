use std::fs::File;
use std::io::{BufReader, Read, Seek};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::data::netcdf_data::NetCdfData;
use crate::netcdf::document::netcdf_document::NetCdfDocument;
use crate::netcdf::header::netcdf_header_reader::NetCdfHeaderReader;

pub struct NetCdfDocumentReader;


impl NetCdfDocumentReader {
    pub fn open_file(filename: &str) -> Result<NetCdfDocument, NetCdfError> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        return Self::open_reader(&mut reader);
    }


    pub fn open_reader<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCdfDocument, NetCdfError> {
        let header = NetCdfHeaderReader::read(reader)?;
        let data = NetCdfData::new();

        let doc = NetCdfDocument::new(
            header,
            data
        );

        return Ok(doc);
    }
}
