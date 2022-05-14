use std::io::{BufReader, Read, Seek};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::string_reader::StringReader;
use crate::grib2::section8::section8::Section8;

pub struct Section8Reader;


impl Section8Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Section8, Grib2Error> {
        let magic = StringReader::read_n_chars(reader, 4)?;
        let section8 = Section8::new(magic)?;

        return Ok(section8);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section8::section8_reader::Section8Reader;

    #[test]
    fn it_correctly_parses_section8() {
        let mut reader = BufReader::new(Cursor::new([
            0x37, 0x37, 0x37, 0x37
        ]));

        let result = Section8Reader::read(&mut reader);
        assert!(result.is_ok());

        let section8 = result.unwrap();
        assert_eq!("7777", section8.end_magic);

        assert_eq!(4, reader.stream_position().unwrap())
    }
}
