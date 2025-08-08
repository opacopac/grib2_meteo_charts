use std::io::Read;

use byteorder::ReadBytesExt;

use crate::grib2::common::byte_reader::ByteReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category_reader::MeteoParameterCategoryReader;
use crate::grib2::section4::product_definition_template_4_1::ProductDefinitionTemplate4_1;

pub struct Section4Template4_1Reader;

impl Section4Template4_1Reader {
    pub fn read(reader: &mut impl Read) -> Result<ProductDefinitionTemplate4_1, Grib2Error> {
        let parameter_category = MeteoParameterCategoryReader::read(reader)?;
        let parameter_number = reader.read_u8()?;

        let _ = ByteReader::read_n_bytes(reader, 26)?; // skip

        let tpl_4_0 = ProductDefinitionTemplate4_1::new(parameter_category, parameter_number);

        Ok(tpl_4_0)
    }
}
