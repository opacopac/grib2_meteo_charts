use std::fs::File;
use std::io::BufReader;

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
    pub fn read(reader: &mut BufReader<File>) -> Result<Section4, Grib2Error> {
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


    fn read_product_definition_template(reader: &mut BufReader<File>) -> Result<ProductDefinitionTemplate, Grib2Error> {
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
