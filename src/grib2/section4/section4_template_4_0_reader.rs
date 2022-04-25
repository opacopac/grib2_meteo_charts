use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::scale_factor_value::ScaleFactorValue;
use crate::grib2::common::scale_factor_value_reader::ScaleFactorValueReader;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

pub struct Section4Template4_0Reader;


impl Section4Template4_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<ProductDefinitionTemplate4_0, Grib2Error> {
        let parameter_category = Section4Template4_0Reader::read_parameter_category(reader)?;
        let parameter_number = reader.read_u8()?;
        let generating_process_type = reader.read_u8()?;
        let generating_process_identifier = reader.read_u8()?;
        let generating_process = reader.read_u8()?;
        let hours_cutoff = reader.read_u16::<BigEndian>()?;
        let mins_cutoff = reader.read_u8()?;
        let forecast_time_unit = reader.read_u8()?;
        let forecast_time_value = reader.read_u8()?;
        let fixed_surface1_type = reader.read_u8()?;
        let fixed_surface1_scale_factor_value = ScaleFactorValueReader::read(reader)?;
        let fixed_surface2_type = reader.read_u8()?;
        let fixed_surface2_scale_factor_value: ScaleFactorValue = ScaleFactorValueReader::read(reader)?;


        let tpl_4_0 = ProductDefinitionTemplate4_0::new(
            parameter_category,
            parameter_number,
            generating_process_type,
            generating_process_identifier,
            generating_process,
            hours_cutoff,
            mins_cutoff,
            forecast_time_unit,
            forecast_time_value,
            fixed_surface1_type,
            fixed_surface1_scale_factor_value,
            fixed_surface2_type,
            fixed_surface2_scale_factor_value
        );

        return Ok(tpl_4_0);
    }


    fn read_parameter_category(reader: &mut BufReader<File>) -> Result<MeteoParameterCategory, Grib2Error> {
        let cat_nr = reader.read_u8()?;
        let meteo_parameter_category = match cat_nr {
            1 => MeteoParameterCategory::Moisture,
            6 => MeteoParameterCategory::Cloud,
            255 => MeteoParameterCategory::Missing,
            _ => MeteoParameterCategory::Unknown(cat_nr)
        };

        return Ok(meteo_parameter_category);
    }
}
