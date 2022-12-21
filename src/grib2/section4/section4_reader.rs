use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;
use crate::grib2::section4::section4::Section4;
use crate::grib2::section4::section4_template_4_0_reader::Section4Template4_0Reader;
use crate::grib2::section4::section4_template_4_8_reader::Section4Template4_8Reader;

pub struct Section4Reader;


impl Section4Reader {
    pub fn read(reader: &mut impl Read) -> Result<Section4, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let coordinate_values_count = reader.read_u16::<BigEndian>()?;
        let product_definition_template = Section4Reader::read_product_definition_template(reader)?;
        let mut coordinate_values = vec![0 as f32; coordinate_values_count as usize];
        reader.read_f32_into::<BigEndian>(&mut coordinate_values)?;

        let section4 = Section4::new(
            length,
            section_number,
            coordinate_values,
            product_definition_template
        )?;

        return Ok(section4);
    }


    fn read_product_definition_template(reader: &mut impl Read) -> Result<ProductDefinitionTemplate, Grib2Error> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let prod_def_tpl = match tpl_number {
            0 => {
                let tpl = Section4Template4_0Reader::read(reader)?;
                ProductDefinitionTemplate::Template4_0(tpl)
            },
            8 => {
                let tpl = Section4Template4_8Reader::read(reader)?;
                ProductDefinitionTemplate::Template4_8(tpl)
            },
            _ => return Err(Grib2Error::InvalidData(
                format!("unsupported product definition template: {}", tpl_number)
            ))
        };

        return Ok(prod_def_tpl);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section4::product_definition_template::ProductDefinitionTemplate;
    use crate::grib2::section4::section4_reader::Section4Reader;

    #[test]
    fn it_correctly_parses_section4_with_a_tpl_4_0() {
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
        assert_eq!(0, section4.coordinate_values.len());

        match section4.product_definition_template {
            ProductDefinitionTemplate::Template4_0(_tpl) => {},
            _ => panic!("wrong product definition template {:?}", section4.product_definition_template)
        };

        assert_eq!(34, reader.stream_position().unwrap())
    }

    #[test]
    fn it_correctly_parses_section4_with_a_tpl_4_8() {
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
        assert_eq!(0, section4.coordinate_values.len());

        match section4.product_definition_template {
            ProductDefinitionTemplate::Template4_8(_tpl) => {},
            _ => panic!("wrong product definition template {:?}", section4.product_definition_template)
        };

        assert_eq!(section4.length as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_section4_with_coordinate_values_after_tpl() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x3A, 0x04, 0x00, 0x06, 0x00, 0x00, 0x03, 0x06, 0x02, 0x00, 0x0B, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x00, 0x00, 0x00, 0x00, 0x01, 0x65, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x42, 0x84, 0x00, 0x00, 0x40, 0x80, 0x00, 0x00, 0xBD, 0x5C, 0xDE, 0xAE, 0x09, 0x7E,
            0xDC, 0xB9, 0x01, 0x3D, 0xC2, 0x85, 0xE5, 0x5B, 0x2D, 0x80
        ]));

        let result = Section4Reader::read(&mut reader);
        assert!(result.is_ok());

        let section4 = result.unwrap();
        assert_eq!(58, section4.length);
        assert_eq!(4, section4.section_number);
        assert_eq!(6, section4.coordinate_values.len());

        match section4.product_definition_template {
            ProductDefinitionTemplate::Template4_0(_tpl) => {},
            _ => panic!("wrong product definition template {:?}", section4.product_definition_template)
        };

        assert_eq!(section4.length as u64, reader.stream_position().unwrap())
    }
}
