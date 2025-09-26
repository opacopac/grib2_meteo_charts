use crate::netcdf::common::netcdf_error::NetCdfError;
use crate::netcdf::common::netcdf_values::NetCdfValues;
use crate::netcdf::common::netcdf_values_reader::NetCdfValuesReader;
use crate::netcdf::header::netcdf_header::NetCdfHeader;
use crate::netcdf::header::netcdf_var::NetCdfVar;
use std::collections::HashMap;
use std::io::{BufReader, Read, Seek, SeekFrom};


pub struct NetCdfDataReader;


impl NetCdfDataReader {
    pub fn read_data_map<T: Read + Seek>(reader: &mut BufReader<T>, header: &NetCdfHeader, var_names: Vec<&str>) -> Result<HashMap<String, NetCdfValues>, NetCdfError> {
        let mut data_map = HashMap::new();
        for var_name in var_names {
            let data = Self::read_data_by_var(reader, header, var_name)?;
            data_map.insert(var_name.to_string(), data);
        }

        Ok(data_map)
    }


    fn read_data_by_var<T: Read + Seek>(reader: &mut BufReader<T>, header: &NetCdfHeader, var_name: &str) -> Result<NetCdfValues, NetCdfError> {
        let var_idx = Self::get_variable_idx(header, var_name)?;
        let variable = &header.var_list[var_idx];
        let entry_count = Self::get_entry_count(&variable);

        let seek_from = SeekFrom::Start(variable.begin);
        reader.seek(seek_from)?;

        let values = NetCdfValuesReader::read(reader, entry_count, &variable.nc_type)?;

        Ok(values)
    }


    fn get_variable_idx(header: &NetCdfHeader, var_name: &str) -> Result<usize, NetCdfError> {
        for i in 0..header.var_list.len() {
            if header.var_list[i].name == var_name {
                return Ok(i);
            }
        }

        Err(NetCdfError::InvalidData(format!("variable '{}' not found!", var_name)))
    }


    fn get_entry_count(var: &NetCdfVar) -> usize {
        let var_size = var.nc_type.get_byte_size() as usize;
        let entry_count = var.size as usize / var_size;

        entry_count
    }
}
