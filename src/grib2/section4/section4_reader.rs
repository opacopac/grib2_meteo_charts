use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;
use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;
use crate::grib2::section4::product_definition_template_4_8::ProductDefinitionTemplate4_8;
use crate::grib2::section4::section4::Section4;
use crate::grib2::section4::section4_template_4_0_reader::Section4Template4_0Reader;
use crate::grib2::section4::section4_template_4_8_reader::Section4Template4_8Reader;

pub struct Section4Reader;


impl Section4Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Section4, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let coordinate_values = reader.read_u16::<BigEndian>()?;
        let product_definition_template = Section4Reader::read_product_definition_template(reader)?;

        let seek = match &product_definition_template {
            ProductDefinitionTemplate::Template4_0(_tpl) => length - 9 - ProductDefinitionTemplate4_0::TPL_LENGTH_BYTES,
            ProductDefinitionTemplate::Template4_8(_tpl) => length - 9 - ProductDefinitionTemplate4_8::TPL_LENGTH_BYTES,
            _ => length - 9
        };
        reader.seek_relative(seek as i64)?;

        let section4 = Section4::new(
            length,
            section_number,
            coordinate_values,
            product_definition_template
        )?;

        return Ok(section4);
    }


    fn read_product_definition_template<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<ProductDefinitionTemplate, Grib2Error> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let grid_def_tpl_type = match tpl_number {
            0 => {
                let tpl = Section4Template4_0Reader::read(reader)?;
                ProductDefinitionTemplate::Template4_0(tpl)
            },
            8 => {
                let tpl = Section4Template4_8Reader::read(reader)?;
                ProductDefinitionTemplate::Template4_8(tpl)
            },
            65535 => ProductDefinitionTemplate::Missing,
            _ => ProductDefinitionTemplate::Unknown(tpl_number)
        };

        return Ok(grid_def_tpl_type);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;
    use crate::grib2::section4::section4_reader::Section4Reader;

    #[test]
    fn it_correctly_parses_a_section4_with_a_tpl40() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x22, 0x04, 0x00, 0x00, 0x00, 0x00, 0x06, 0xC7, 0x02, 0x00, 0x0B, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF
        ]));

        let result = Section4Reader::read(&mut reader);
        assert!(result.is_ok());

        let section4 = result.unwrap();
        assert_eq!(34, section4.length);
        assert_eq!(4, section4.section_number);
        assert_eq!(0, section4.coordinate_values);

        match section4.product_definition_template {
            ProductDefinitionTemplate::Template4_0(_tpl) => {},
            _ => panic!("wrong product definition template {:?}", section4.product_definition_template)
        };
    }

    #[test]
    fn it_correctly_parses_a_section4_with_a_tpl48() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x3A, 0x04, 0x00, 0x00, 0x00, 0x08, 0x01, 0x34, 0x02, 0x00, 0x0B, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0x07, 0xE6, 0x04, 0x19, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
            0x00, 0x00, 0x00, 0x00, 0x3C, 0xFF, 0x00, 0x00, 0x00, 0x00
        ]));

        let result = Section4Reader::read(&mut reader);
        assert!(result.is_ok());

        let section4 = result.unwrap();
        assert_eq!(58, section4.length);
        assert_eq!(4, section4.section_number);
        assert_eq!(0, section4.coordinate_values);

        match section4.product_definition_template {
            ProductDefinitionTemplate::Template4_8(_tpl) => {},
            _ => panic!("wrong product definition template {:?}", section4.product_definition_template)
        };
    }
}
