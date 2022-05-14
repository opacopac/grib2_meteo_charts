use crate::netcdf::data::netcdf_data::NetCdfData;
use crate::netcdf::header::netcdf_header::NetCdfHeader;

pub struct NetCdfDocument {
    pub header: NetCdfHeader,
    pub data: NetCdfData
}
