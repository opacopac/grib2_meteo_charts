use std::fs::File;
use std::io::{BufReader, Read};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::section0_reader::Section0Reader;
use crate::grib2::section1::section1_reader::Section1Reader;
use crate::grib2::section2::section2_reader::Section2Reader;
use crate::grib2::section3::section3_reader::Section3Reader;
use crate::grib2::section4::section4_reader::Section4Reader;
use crate::grib2::section5::section5_reader::Section5Reader;
use crate::grib2::section6::section6_reader::Section6Reader;
use crate::grib2::section7::section7_reader::Section7Reader;
use crate::grib2::section8::section8_reader::Section8Reader;
use peekbufread::PeekRead;


pub struct Grib2DocumentReader;


impl Grib2DocumentReader {
    pub fn read_single_doc_from_file(filename: &str) -> Result<Grib2Document, Grib2Error> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        Self::read_single_doc_from_stream(&mut reader)
    }


    pub fn read_multi_doc_from_file(filename: &str) -> Result<Vec<Grib2Document>, Grib2Error> {
        let file = File::open(filename)?;
        let mut reader = PeekRead::new(file);
        let mut documents = Vec::new();

        loop {
            let doc = Self::read_single_doc_from_stream(&mut reader)?;
            documents.push(doc);

            // Check if we have reached the end of the file
            let mut buf = [0; 1];
            let peeked_bytes = reader.peek(&mut buf).unwrap_or(0);
            if (peeked_bytes == 0) {
                break;
            }
        }

        Ok(documents)
    }


    pub fn read_single_doc_from_stream(reader: &mut impl Read) -> Result<Grib2Document, Grib2Error> {
        let section0 = Section0Reader::read(reader)?;
        let section1 = Section1Reader::read(reader)?;
        let section2 = Section2Reader::read(reader)?;
        let section3 = Section3Reader::read(reader)?;
        let section4 = Section4Reader::read(reader)?;
        let section5 = Section5Reader::read(reader)?;
        let section6 = Section6Reader::read(reader)?;
        let section7 = Section7Reader::read(reader)?;
        let section8 = Section8Reader::read(reader)?;
        let document = Grib2Document::new(
            section0,
            section1,
            section2,
            section3,
            section4,
            section5,
            section6,
            section7,
            section8,
        );

        return Ok(document);
    }
}
