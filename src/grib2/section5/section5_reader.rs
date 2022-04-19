use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};

use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate;
use crate::grib2::section5::data_representation_template_5_0_reader::DataRepresentationTemplate5_0Reader;
use crate::grib2::section5::section5::Section5;

pub struct Section5Reader;


impl Section5Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section5, Grib2Error> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let data_points = reader.read_u32::<BigEndian>()?;
        let data_representation_template = Section5Reader::read_data_representation_template(reader)?;
        let section5 = Section5::new(
            length,
            section_number,
            data_points,
            data_representation_template
        )?;

        reader.seek_relative(length as i64 - 21)?; // TODO: temp

        return Ok(section5);
    }


    fn read_data_representation_template(reader: &mut BufReader<File>) -> Result<DataRepresentationTemplate, Grib2Error> {
        let tpl_number = reader.read_u16::<BigEndian>()?;
        let data_rep_tpl = match tpl_number {
            0 => {
                let tpl = DataRepresentationTemplate5_0Reader::read(reader)?;
                DataRepresentationTemplate::GridPointDataSimplePacking(tpl)
            },
            65535 => DataRepresentationTemplate::Missing,
            _ => DataRepresentationTemplate::Unknown(tpl_number)
        };

        return Ok(data_rep_tpl);
    }
}
