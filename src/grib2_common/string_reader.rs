use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::from_utf8;

pub struct StringReader;


impl StringReader {
    pub fn read_4_chars(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;

        let text = from_utf8(&buf)?.to_string();

        return Ok(text);
    }
}
