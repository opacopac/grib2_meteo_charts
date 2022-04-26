use std::fs::File;
use std::io::BufReader;

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::product_definition_template_4_8::ProductDefinitionTemplate4_8;
use crate::grib2::section4::section4_template_4_0_reader::Section4Template4_0Reader;

pub struct Section4Template4_8Reader;


impl Section4Template4_8Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<ProductDefinitionTemplate4_8, Grib2Error> {
        let parameter_category = Section4Template4_0Reader::read_parameter_category(reader)?;
        let parameter_number = reader.read_u8()?;


        let tpl = ProductDefinitionTemplate4_8::new(
            parameter_category,
            parameter_number,
        );

        return Ok(tpl);
    }
}
