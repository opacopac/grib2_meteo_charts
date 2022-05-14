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

        if version != 2 {
            return Err(NetCdfError::InvalidData(
                format!("Unsupported version {}, expected: {}", version, 2)
            ));
        }

        return Ok(NetCdfMagic {
            magic,
            version
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::netcdf::header::netcfd_magic::{NETCDF_MAGIC, NetCdfMagic};

    #[test]
    fn it_verifies_the_correct_magic() {
        let result = NetCdfMagic::new("TODO".to_string(), 2);
        assert!(result.is_err());
    }


    #[test]
    fn it_verifies_the_supported_version() {
        let result = NetCdfMagic::new(NETCDF_MAGIC.to_string(), 3);
        assert!(result.is_err());
    }
}
