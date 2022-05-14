use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section2::section2::Section2;

pub struct Section2Reader;


impl Section2Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Section2, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let section2 = Section2::new(
            length,
            section_number,
        )?;

        reader.seek_relative(length as i64 - 5)?;

        return Ok(section2);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section2::section2_reader::Section2Reader;

    #[test]
    fn it_correctly_parses_section2() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x1B, 0x02, 0xFE, 0x00, 0x07, 0xE6, 0x04, 0x11, 0x00, 0x29, 0x38, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
        ]));

        let result = Section2Reader::read(&mut reader);
        assert!(result.is_ok());

        let section2 = result.unwrap();
        assert_eq!(27, section2.length);
        assert_eq!(2, section2.section_number);

        assert_eq!(section2.length as u64, reader.stream_position().unwrap())
    }
}
