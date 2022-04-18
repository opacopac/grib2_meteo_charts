use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2_common::grib2_error::Grib2Error;
use crate::grib2_section5::data_representation_template_5_0::DataRepresentationTemplate5_0;
use crate::grib2_section5::original_field_type::OriginalFieldType;

pub struct DataRepresentationTemplate5_0Reader;


impl DataRepresentationTemplate5_0Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<DataRepresentationTemplate5_0, Grib2Error> {
        let reference_value = reader.read_u32::<BigEndian>()?;
        let binary_scale_factor = reader.read_u16::<BigEndian>()?;
        let decimal_scale_factor = reader.read_u16::<BigEndian>()?;
        let number_of_bits = reader.read_u8()?;
        let original_field_type = DataRepresentationTemplate5_0Reader::read_original_field_type(reader)?;
        let tpl = DataRepresentationTemplate5_0::new(
            reference_value,
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits,
            original_field_type
        );

        return Ok(tpl);
    }


    fn read_original_field_type(reader: &mut BufReader<File>) -> Result<OriginalFieldType, Grib2Error> {
        let value = reader.read_u8()?;
        let original_field_type = match value {
            0 => OriginalFieldType::FloatingPoint,
            1 => OriginalFieldType::Integer,
            255 => OriginalFieldType::Missing,
            _ => OriginalFieldType::Unknown(value)
        };

        return Ok(original_field_type);
    }
}
