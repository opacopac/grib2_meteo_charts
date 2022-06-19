use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section7::section7::Section7;

pub struct Section7Reader;


impl Section7Reader {
    pub fn read(reader: &mut impl Read) -> Result<Section7, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let num_data_points = ((length - 5) / 2) as usize; // TODO: dependent on bits in sect 5
        let data_points = Section7Reader::read_data_points(reader, num_data_points)?;
        let section7 = Section7::new(
            length,
            section_number,
            data_points
        )?;

        return Ok(section7);
    }


    fn read_data_points(reader: &mut impl Read, num_data_points: usize) -> Result<Vec<u16>, Grib2Error> {
        let mut buf: Vec<u16> = vec![0; num_data_points];

        reader.read_u16_into::<BigEndian>(&mut buf)?;

        return Ok(buf);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};

    use crate::grib2::section7::section7_reader::Section7Reader;

    #[test]
    fn it_correctly_parses_section7() {
        let mut reader = BufReader::new(Cursor::new([
            0x00, 0x00, 0x00, 0x0F, 0x07, 0x79, 0x45, 0x79, 0x84, 0x79, 0x8F, 0x79, 0x51, 0x79, 0x23
        ]));

        let result = Section7Reader::read(&mut reader);
        assert!(result.is_ok());

        let section7 = result.unwrap();
        assert_eq!(15, section7.length);
        assert_eq!(7, section7.section_number);

        assert_eq!(section7.length as u64, reader.stream_position().unwrap())
    }
}
