use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate;
use crate::grib2::section5::data_representation_template_5_0_reader::DataRepresentationTemplate5_0Reader;
use crate::grib2::section5::section5::Section5;

pub struct Section5Reader;


impl Section5Reader {
    pub fn read(reader: &mut impl Read) -> Result<Section5, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let data_points = reader.read_u32::<BigEndian>()?;
        let data_representation_template = Section5Reader::read_data_representation_template(reader)?;
        let section5 = Section5::new(
            length,
            section_number,
            data_points,
            data_representation_template
        )?;

        return Ok(section5);
    }


    fn read_data_representation_template(reader: &mut impl Read) -> Result<DataRepresentationTemplate, Grib2Error> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let data_rep_tpl = match tpl_number {
            0 => {
                let tpl = DataRepresentationTemplate5_0Reader::read(reader)?;
                DataRepresentationTemplate::GridPointDataSimplePacking(tpl)
            },
            _ => return Err(Grib2Error::InvalidData(
                format!("unsupported data representation template: {}", tpl_number)
            ))
        };

        return Ok(data_rep_tpl);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section5::data_representation_template::DataRepresentationTemplate;
    use crate::grib2::section5::section5_reader::Section5Reader;

    #[test]
    fn it_correctly_parses_section5() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x15, 0x05, 0x00, 0x0B, 0x84, 0xAE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
            0x0F, 0x00, 0x00, 0x10, 0x00
        ]));

        let result = Section5Reader::read(&mut reader);
        assert!(result.is_ok());

        let section5 = result.unwrap();
        assert_eq!(21, section5.length);
        assert_eq!(5, section5.section_number);
        assert_eq!(754862, section5.data_points);

        match section5.data_representation_template {
            DataRepresentationTemplate::GridPointDataSimplePacking(_tpl) => {},
            _ => panic!("wrong data representation template {:?}", section5.data_representation_template)
        };

        assert_eq!(section5.length as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_a_section5_for_data_rep_tpl_23774() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x15, 0x05, 0x00, 0x0B, 0x84, 0xAE, 0x00, 0x00, 0xC2, 0x4F, 0xBA, 0x60, 0x80,
            0x03, 0x00, 0x00, 0x10, 0x00
        ]));

        let result = Section5Reader::read(&mut reader);
        assert!(result.is_ok());

        let section5 = result.unwrap();
        assert_eq!(21, section5.length);
        assert_eq!(5, section5.section_number);
        assert_eq!(754862, section5.data_points);

        match section5.data_representation_template {
            DataRepresentationTemplate::GridPointDataSimplePacking(_tpl) => {},
            _ => panic!("wrong data representation template {:?}", section5.data_representation_template)
        };

        assert_eq!(section5.length as u64, reader.stream_position().unwrap())
    }
}
