use std::io::Read;

use byteorder::ReadBytesExt;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

pub struct MeteoParameterCategoryReader;


impl MeteoParameterCategoryReader {
    pub fn read(reader: &mut impl Read) -> Result<MeteoParameterCategory, Grib2Error> {
        let cat_nr = reader.read_u8()?;
        let meteo_parameter_category = match cat_nr {
            1 => MeteoParameterCategory::Moisture,
            2 => MeteoParameterCategory::Momentum,
            3 => MeteoParameterCategory::Mass,
            6 => MeteoParameterCategory::Cloud,
            191 => MeteoParameterCategory::Miscellaneous,
            255 => MeteoParameterCategory::Missing,
            _ => MeteoParameterCategory::Unknown(cat_nr)
        };

        return Ok(meteo_parameter_category);
    }
}
