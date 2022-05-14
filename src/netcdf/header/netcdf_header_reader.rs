pub struct NetCdfHeaderReader;


impl NetCdfHeaderReader {
    /*pub fn read<T: Read + Seek>(reader: &mut BufReader<T>) -> Result<NetCfdHeader, NetCdfError> {
        let magic = StringReader::read_4_chars(reader)?;
        reader.seek_relative(2)?; // 2 reserved bytes

        let header = NetCfdHeader::new()?;

        return Ok(header);
    }*/
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    #[test]
    fn it_correctly_parses_the_header() {
        let mut reader = BufReader::new(Cursor::new([
            0x47, 0x52, 0x49, 0x42, 0xFF, 0xFF, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0xC4, 0xBD
        ]));

        /*let result = NetCfdHeaderReader::read(&mut reader);
        assert!(result.is_ok());*/

        /*let section0 = result.unwrap();
        assert_eq!("GRIB", section0.magic);
        assert_eq!(Discipline::Meteorological, section0.discipline);
        assert_eq!(2, section0.edition);
        assert_eq!(1623229, section0.length);*/
    }
}
