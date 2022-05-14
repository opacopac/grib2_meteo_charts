use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::common::string_reader::StringReader;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section0::section0::Section0;

pub struct Section0Reader;


impl Section0Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<Section0, Grib2Error> {
        let magic = StringReader::read_n_chars(reader, 4)?;
        reader.seek_relative(2)?; // 2 reserved bytes
        let discipline = Section0Reader::read_discipline(reader)?;
        let edition = reader.read_u8()?;
        let length = reader.read_u64::<BigEndian>()?;

        let section0 = Section0::new(
            magic,
            discipline,
            edition,
            length
        )?;

        return Ok(section0);
    }


    fn read_discipline<T: Read>(reader: &mut BufReader<T>) -> Result<Discipline, Grib2Error> {
        let value = reader.read_u8()?;
        let discipline = match value {
            0 => Discipline::Meteorological,
            1 => Discipline::Hydrological,
            2 => Discipline::LandSurface,
            3 => Discipline::SatelliteRemoteSensing,
            4 => Discipline::SpaceWeather,
            10 => Discipline::Oceanographic,
            255 => Discipline::Missing,
            _ => Discipline::Unknown(value)
        };

        return Ok(discipline);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Seek};
    use crate::grib2::section0::discipline::Discipline;
    use crate::grib2::section0::section0_reader::Section0Reader;

    #[test]
    fn it_correctly_parses_section0() {
        let mut reader = BufReader::new(Cursor::new([
            0x47, 0x52, 0x49, 0x42, 0xFF, 0xFF, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0xC4, 0xBD
        ]));

        let result = Section0Reader::read(&mut reader);
        assert!(result.is_ok());

        let section0 = result.unwrap();
        assert_eq!("GRIB", section0.magic);
        assert_eq!(Discipline::Meteorological, section0.discipline);
        assert_eq!(2, section0.edition);
        assert_eq!(1623229, section0.length);

        assert_eq!(16 as u64, reader.stream_position().unwrap())
    }
}
