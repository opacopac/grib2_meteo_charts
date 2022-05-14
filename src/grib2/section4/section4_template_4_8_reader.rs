use std::io::{BufReader, Read, Seek};

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category_reader::MeteoParameterCategoryReader;
use crate::grib2::section4::product_definition_template_4_8::ProductDefinitionTemplate4_8;

pub struct Section4Template4_8Reader;


impl Section4Template4_8Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<ProductDefinitionTemplate4_8, Grib2Error> {
        let parameter_category = MeteoParameterCategoryReader::read(reader)?;
        let parameter_number = reader.read_u8()?;

        reader.seek_relative(30)?; // skip

        let num_of_time_range_specs = reader.read_u8()?;
        let skip_seek = 4 + 12 * num_of_time_range_specs as i64;

        reader.seek_relative(skip_seek)?; // skip

        let tpl_4_8 = ProductDefinitionTemplate4_8::new(
            parameter_category,
            parameter_number,
        );

        return Ok(tpl_4_8);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
    use crate::grib2::section4::section4_template_4_0_reader::Section4Template4_0Reader;

    #[test]
    fn it_correctly_parses_a_template_4_8() {
        let mut reader = BufReader::new(Cursor::new([
            0x01, 0x34, 0x02, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xE6, 0x04, 0x19, 0x01, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x3C, 0xFF, 0x00, 0x00, 0x00,
            0x00
        ]));

        let result = Section4Template4_0Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl40 = result.unwrap();
        assert_eq!(MeteoParameterCategory::Moisture, tpl40.parameter_category);
        assert_eq!(52, tpl40.parameter_number);
    }
}
