use std::io::Read;

use byteorder::ReadBytesExt;

use crate::grib2::common::byte_reader::ByteReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category_reader::MeteoParameterCategoryReader;
use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

pub struct Section4Template4_0Reader;


impl Section4Template4_0Reader {
    pub fn read(reader: &mut impl Read) -> Result<ProductDefinitionTemplate4_0, Grib2Error> {
        let parameter_category = MeteoParameterCategoryReader::read(reader)?;
        let parameter_number = reader.read_u8()?;

        let _ = ByteReader::read_n_bytes(reader, 23)?; // skip

        let tpl_4_0 = ProductDefinitionTemplate4_0::new(
            parameter_category,
            parameter_number,
        );

        return Ok(tpl_4_0);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
    use crate::grib2::section4::section4_template_4_0_reader::Section4Template4_0Reader;

    #[test]
    fn it_correctly_parses_template_4_0() {
        let mut reader = BufReader::new(Cursor::new([
            0x06, 0xC7, 0x02, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
        ]));

        let result = Section4Template4_0Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl40 = result.unwrap();
        assert_eq!(MeteoParameterCategory::Cloud, tpl40.parameter_category);
        assert_eq!(199, tpl40.parameter_number);
    }


    #[test]
    fn it_correctly_parses_template_4_0_of_a_hhl_layer() {
        let mut reader = BufReader::new(Cursor::new([
            0x03, 0x06, 0x02, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00
        ]));

        let result = Section4Template4_0Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl40 = result.unwrap();
        assert_eq!(MeteoParameterCategory::Mass, tpl40.parameter_category);
        assert_eq!(6, tpl40.parameter_number);
    }
}
