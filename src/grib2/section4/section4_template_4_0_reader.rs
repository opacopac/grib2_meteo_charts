use std::fs::File;
use std::io::BufReader;

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

pub struct Section4Template4_0Reader;


impl Section4Template4_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<ProductDefinitionTemplate4_0, Grib2Error> {
        let parameter_category = reader.read_u8()?;
        let parameter_number = reader.read_u8()?;
        let generating_process_type = reader.read_u8()?;
        let generating_process_identifier = reader.read_u8()?;
        let generating_process = reader.read_u8()?;

        let tpl_4_0 = ProductDefinitionTemplate4_0::new(
            parameter_category,
            parameter_number,
            generating_process_type,
            generating_process_identifier,
            generating_process
        );

        return Ok(tpl_4_0);
    }
}
