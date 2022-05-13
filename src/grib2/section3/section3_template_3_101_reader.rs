use std::io::{BufReader, Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section3::grid_definition_template_3_101::GridDefinitionTemplate3_101;
use crate::grib2::section3::shape_of_earth_reader::ShapeOfEarthReader;

pub struct Section3Template3_101Reader;


impl Section3Template3_101Reader {
    pub fn read<T: Read+Seek>(reader: &mut BufReader<T>) -> Result<GridDefinitionTemplate3_101, Grib2Error> {
        let shape_of_earth = ShapeOfEarthReader::read(reader)?;
        let number_of_grid = reader.read_u24::<BigEndian>()?;
        let number_of_grid_in_ref = reader.read_u8()?;
        let hor_grid_uuid = reader.read_u128::<BigEndian>()?;
        let tpl_3_101 = GridDefinitionTemplate3_101::new(
            shape_of_earth,
            number_of_grid,
            number_of_grid_in_ref,
            hor_grid_uuid
        );

        return Ok(tpl_3_101);
    }
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::grib2::section3::section3_template_3_101_reader::Section3Template3_101Reader;
    use crate::grib2::section3::shape_of_earth::ShapeOfEarth;

    #[test]
    fn it_correctly_parses_a_template_3_101() {
        let mut reader = BufReader::new(Cursor::new([
            0x06, 0x00, 0x00, 0x1A, 0x01, 0xA2, 0x7B, 0x8D, 0xE6, 0x18, 0xC4, 0x11, 0xE4, 0x82, 0x0A, 0xB5,
            0xB0, 0x98, 0xC6, 0xA5, 0xC0
        ]));

        let result = Section3Template3_101Reader::read(&mut reader);
        assert!(result.is_ok());

        let tpl3101 = result.unwrap();
        assert_eq!(ShapeOfEarth::SphericalRadius6371229, tpl3101.shape_of_earth);
        assert_eq!(26, tpl3101.number_of_grid);
        assert_eq!(1, tpl3101.number_of_grid_in_ref);
        assert_eq!(0xA27B8DE618C411E4820AB5B098C6A5C0, tpl3101.hor_grid_uuid);
    }
}
