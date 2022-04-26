use std::fs::File;
use std::io::BufReader;

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

pub struct Section4Template4_0Reader;


impl Section4Template4_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<ProductDefinitionTemplate4_0, Grib2Error> {
        let parameter_category = Section4Template4_0Reader::read_parameter_category(reader)?;
        let parameter_number = reader.read_u8()?;

        let tpl_4_0 = ProductDefinitionTemplate4_0::new(
            parameter_category,
            parameter_number,
        );

        return Ok(tpl_4_0);
    }


    pub fn read_parameter_category(reader: &mut BufReader<File>) -> Result<MeteoParameterCategory, Grib2Error> {
        let cat_nr = reader.read_u8()?;
        let meteo_parameter_category = match cat_nr {
            1 => MeteoParameterCategory::Moisture,
            2 => MeteoParameterCategory::Momentum,
            6 => MeteoParameterCategory::Cloud,
            255 => MeteoParameterCategory::Missing,
            _ => MeteoParameterCategory::Unknown(cat_nr)
        };

        return Ok(meteo_parameter_category);
    }
}
