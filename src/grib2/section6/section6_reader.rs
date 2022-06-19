use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::byte_reader::ByteReader;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section6::section6::Section6;

pub struct Section6Reader;


impl Section6Reader {
    pub fn read(reader: &mut impl Read) -> Result<Section6, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let bitmap_indicator = reader.read_u8()?;
        let bitmap = ByteReader::read_n_bytes(reader, (length - 6) as usize)?;

        let section6 = Section6::new(
            length,
            section_number,
            bitmap_indicator,
            bitmap
        )?;

        return Ok(section6);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section6::section6_reader::Section6Reader;

    #[test]
    fn it_correctly_parses_section6_without_a_bitmap() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x06, 0x06, 0xFF
        ]));

        let result = Section6Reader::read(&mut reader);
        assert!(result.is_ok());

        let section6 = result.unwrap();
        assert_eq!(6, section6.length);
        assert_eq!(6, section6.section_number);
        assert_eq!(255, section6.bitmap_indicator);
        assert_eq!(0, section6.bitmap.len());

        assert_eq!(section6.length as u64, reader.stream_position().unwrap())
    }


    #[test]
    fn it_correctly_parses_section6_with_a_bitmap() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x10, 0x06, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A
        ]));

        let result = Section6Reader::read(&mut reader);
        assert!(result.is_ok());

        let section6 = result.unwrap();
        assert_eq!(16, section6.length);
        assert_eq!(6, section6.section_number);
        assert_eq!(0, section6.bitmap_indicator);
        assert_eq!(10, section6.bitmap.len());
        assert_eq!(0x01, section6.bitmap[0]);

        assert_eq!(section6.length as u64, reader.stream_position().unwrap())
    }
}
