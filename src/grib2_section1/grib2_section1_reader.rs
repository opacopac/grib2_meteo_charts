use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::grib2_section1::grib2_processed_data_type::Grib2ProcessedDataType;
use crate::grib2_section1::grib2_production_status::Grib2ProductionStatus;

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
        let production_status = Grib2Section1Reader::read_production_status(reader)?;
        let processed_data_type = Grib2Section1Reader::read_processed_data_type(reader)?;
        reader.consume(length as usize - 21);
        let section1 = Grib2Section1::new(
            length,
            section_number,
            center,
            subcenter,
            master_table_version,
            local_table_version,
            ref_time_significance,
            ref_time,
            production_status,
            processed_data_type
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


    fn read_production_status(reader: &mut BufReader<File>) -> Result<Grib2ProductionStatus, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let production_status = match value {
            0 => Grib2ProductionStatus::Operational,
            1 => Grib2ProductionStatus::Test,
            2 => Grib2ProductionStatus::Research,
            3 => Grib2ProductionStatus::ReAnalysis,
            4 => Grib2ProductionStatus::Thorpex,
            5 => Grib2ProductionStatus::ThorpexTest,
            6 => Grib2ProductionStatus::S2sOperational,
            7 => Grib2ProductionStatus::S2sTest,
            8 => Grib2ProductionStatus::Uerra,
            9 => Grib2ProductionStatus::UerraTest,
            255 => Grib2ProductionStatus::Missing,
            _ => Grib2ProductionStatus::Unknown(value)
        };

        return Ok(production_status);
    }


    fn read_processed_data_type(reader: &mut BufReader<File>) -> Result<Grib2ProcessedDataType, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let ref_time_significance = match value {
            0 => Grib2ProcessedDataType::Analysis,
            1 => Grib2ProcessedDataType::Forecast,
            2 => Grib2ProcessedDataType::AnalysisAndForecast,
            3 => Grib2ProcessedDataType::ControlForecast,
            4 => Grib2ProcessedDataType::PerturbedForecast,
            5 => Grib2ProcessedDataType::ControlAndPerturbedForecast,
            6 => Grib2ProcessedDataType::ProcessedSatelliteObservations,
            7 => Grib2ProcessedDataType::ProcessedRadarObservations,
            8 => Grib2ProcessedDataType::EventProbability,
            192 => Grib2ProcessedDataType::Experimental,
            255 => Grib2ProcessedDataType::Missing,
            _ => Grib2ProcessedDataType::Unknown(value)
        };

        return Ok(ref_time_significance);
    }
}
