use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::grib2_section1::grib2_ref_time_significance::Grib2RefTimeSignificance;
use crate::grib2_section1::grib2_section1::Grib2Section1;

pub struct Grib2Section1Reader;


impl Grib2Section1Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Grib2Section1, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let center = reader.read_u16::<BigEndian>()?;
        let subcenter = reader.read_u16::<BigEndian>()?;
        let master_table_version = reader.read_u8()?;
        let local_table_version = reader.read_u8()?;
        let ref_time_significance = Grib2Section1Reader::read_ref_time_significance(reader)?;
        let ref_time = Grib2Section1Reader::read_ref_time(reader)?;
        let section1 = Grib2Section1::new(
            length,
            section_number,
            center,
            subcenter,
            master_table_version,
            local_table_version,
            ref_time_significance,
            ref_time
        )?;

        return Ok(section1);
    }


    fn read_ref_time_significance(reader: &mut BufReader<File>) -> Result<Grib2RefTimeSignificance, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let ref_time_significance = match value {
            0 => Grib2RefTimeSignificance::Analysis,
            1 => Grib2RefTimeSignificance::StartOfForecast,
            2 => Grib2RefTimeSignificance::VerifyingTimeOfForecast,
            3 => Grib2RefTimeSignificance::ObservationTime,
            255 => Grib2RefTimeSignificance::Missing,
            _ => Grib2RefTimeSignificance::Unknown(value)
        };

        return Ok(ref_time_significance);
    }


    fn read_ref_time(reader: &mut BufReader<File>) -> Result<NaiveDateTime, Box<dyn Error>> {
        let year = i32::try_from(reader.read_u16::<BigEndian>()?)?;
        let month = reader.read_u8()? as u32;
        let day = reader.read_u8()? as u32;
        let hour = reader.read_u8()? as u32;
        let minute = reader.read_u8()? as u32;
        let second = reader.read_u8()? as u32;
        let ref_time = NaiveDateTime::new(
            NaiveDate::from_ymd(year, month, day),
            NaiveTime::from_hms(hour, minute, second)
        );

        return Ok(ref_time);
    }
}
