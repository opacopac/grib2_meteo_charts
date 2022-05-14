use std::io::{BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::signed_number_reader::SignedNumberReader;
use crate::grib2::section5::data_representation_template_5_0::DataRepresentationTemplate5_0;
use crate::grib2::section5::original_field_type::OriginalFieldType;

pub struct DataRepresentationTemplate5_0Reader;


impl DataRepresentationTemplate5_0Reader {
    pub fn read<T: Read>(reader: &mut BufReader<T>) -> Result<DataRepresentationTemplate5_0, Grib2Error> {
        let reference_value = reader.read_f32::<BigEndian>()?;
        let binary_scale_factor = SignedNumberReader::read(reader)?;
        let decimal_scale_factor = SignedNumberReader::read(reader)?;
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


    fn read_original_field_type<T: Read>(reader: &mut BufReader<T>) -> Result<OriginalFieldType, Grib2Error> {
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


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section5::data_representation_template_5_0_reader::DataRepresentationTemplate5_0Reader;
    use crate::grib2::section5::original_field_type::OriginalFieldType;

    #[test]
    fn it_correctly_parses_template_5_0() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x00, 0x80, 0x0F, 0x00, 0x00, 0x10, 0x00
        ]));

        let result = DataRepresentationTemplate5_0Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl_5_0 = result.unwrap();
        assert_eq!(0.0, tpl_5_0.reference_value);
        assert_eq!(-15, tpl_5_0.binary_scale_factor_e);
        assert_eq!(0, tpl_5_0.decimal_scale_factor_d);
        assert_eq!(16, tpl_5_0.number_of_bits);
        assert_eq!(OriginalFieldType::FloatingPoint, tpl_5_0.original_field_type);

        assert_eq!(10, reader.stream_position().unwrap())
    }
}
