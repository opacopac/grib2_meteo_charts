use crate::netcdf::common::netcdf_error::NetCdfError;

pub struct NetCdfMagic {
    pub magic: String,
    pub version: u8
}

const NETCDF_MAGIC: &str = "CDF";

impl NetCdfMagic {
    pub fn new(
        magic: String,
        version: u8
    ) -> Result<NetCdfMagic, NetCdfError> {
        if magic != NETCDF_MAGIC {
            return Err(NetCdfError::InvalidData(
                format!("Invalid magic {}, expected: {}", magic, NETCDF_MAGIC)
            ));
        }

        return Ok(NetCdfMagic {
            magic,
            version
        });
    }
}
