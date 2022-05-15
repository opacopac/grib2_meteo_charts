use std::io::{BufReader, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::document::netcdf_document::NetCdfDocument;
use crate::netcdf::header::netcdf_var::NetCdfVar;

pub struct NetCdfDataReader;


impl NetCdfDataReader {
    pub fn read_data_by_var<T: Read+Seek>(reader: &mut BufReader<T>, doc: &NetCdfDocument, var_name: &str) -> Result<Vec<f64>, NetCdfError> {
        let var_idx = Self::get_variable_idx(doc, var_name)?;
        let variable = &doc.header.var_list[var_idx];
        let entry_count = Self::get_entry_count(&variable);

        let seek_from = SeekFrom::Start(variable.begin);
        reader.seek(seek_from)?;

        let mut buf = vec![0.0; entry_count];
        reader.read_f64_into::<BigEndian>(&mut buf)?;

        return Ok(buf);
    }


    fn get_variable_idx(doc: &NetCdfDocument, var_name: &str) -> Result<usize, NetCdfError> {
        for i in 0..doc.header.var_list.len() {
            if doc.header.var_list[i].name == var_name {
                return Ok(i);
            }
        }

        return Err(NetCdfError::InvalidData(
            format!("variable '{}' not found!", var_name)
        ));
    }


    fn get_entry_count(var: &NetCdfVar) -> usize {
        let var_size = var.nc_type.get_byte_size() as usize;
        let entry_count = var.size as usize / var_size;

        return entry_count;
    }
}
