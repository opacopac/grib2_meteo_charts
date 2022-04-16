use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::{from_utf8};

use simple_error::bail;

use crate::grib2_cloud_cover::grib2_section0::Grib2Section0;

pub struct Grib2Section0Reader;


const GRIB2_MAGIC: &str = "GRIB";

impl Grib2Section0Reader {
    pub fn read(reader: BufReader<File>) -> Result<Grib2Section0, Box<dyn Error>> {
        let magic = Grib2Section0Reader::read_magic(reader)?;

        return Ok(Grib2Section0::new(
            magic
        ));
    }


    fn read_magic(mut reader: BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;

        let magic = from_utf8(&buf)?.to_string();

        if magic != GRIB2_MAGIC {
            bail!("Not a GRIB2 file");
        }

        return Ok(magic);
    }
}
