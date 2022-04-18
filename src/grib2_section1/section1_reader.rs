use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::grib2_section1::processed_data_type::ProcessedDataType;
use crate::grib2_section1::production_status::ProductionStatus;

use crate::grib2_section1::ref_time_significance::RefTimeSignificance;
use crate::grib2_section1::section1::Section1;

pub struct Section1Reader;


impl Section1Reader {
    pub fn read(reader: &mut BufReader<File>) -> Result<Section1, Box<dyn Error>> {
        let length = reader.read_u32::<BigEndian>()?;
        let section_number = reader.read_u8()?;
        let center = reader.read_u16::<BigEndian>()?;
        let subcenter = reader.read_u16::<BigEndian>()?;
        let master_table_version = reader.read_u8()?;
        let local_table_version = reader.read_u8()?;
        let ref_time_significance = Section1Reader::read_ref_time_significance(reader)?;
        let ref_time = Section1Reader::read_ref_time(reader)?;
        let production_status = Section1Reader::read_production_status(reader)?;
        let processed_data_type = Section1Reader::read_processed_data_type(reader)?;
        let section1 = Section1::new(
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

        reader.seek_relative(length as i64 - 21)?;

        return Ok(section1);
    }


    fn read_ref_time_significance(reader: &mut BufReader<File>) -> Result<RefTimeSignificance, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let ref_time_significance = match value {
            0 => RefTimeSignificance::Analysis,
            1 => RefTimeSignificance::StartOfForecast,
            2 => RefTimeSignificance::VerifyingTimeOfForecast,
            3 => RefTimeSignificance::ObservationTime,
            255 => RefTimeSignificance::Missing,
            _ => RefTimeSignificance::Unknown(value)
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


    fn read_production_status(reader: &mut BufReader<File>) -> Result<ProductionStatus, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let production_status = match value {
            0 => ProductionStatus::Operational,
            1 => ProductionStatus::Test,
            2 => ProductionStatus::Research,
            3 => ProductionStatus::ReAnalysis,
            4 => ProductionStatus::Thorpex,
            5 => ProductionStatus::ThorpexTest,
            6 => ProductionStatus::S2sOperational,
            7 => ProductionStatus::S2sTest,
            8 => ProductionStatus::Uerra,
            9 => ProductionStatus::UerraTest,
            255 => ProductionStatus::Missing,
            _ => ProductionStatus::Unknown(value)
        };

        return Ok(production_status);
    }


    fn read_processed_data_type(reader: &mut BufReader<File>) -> Result<ProcessedDataType, Box<dyn Error>> {
        let value = reader.read_u8()?;
        let ref_time_significance = match value {
            0 => ProcessedDataType::Analysis,
            1 => ProcessedDataType::Forecast,
            2 => ProcessedDataType::AnalysisAndForecast,
            3 => ProcessedDataType::ControlForecast,
            4 => ProcessedDataType::PerturbedForecast,
            5 => ProcessedDataType::ControlAndPerturbedForecast,
            6 => ProcessedDataType::ProcessedSatelliteObservations,
            7 => ProcessedDataType::ProcessedRadarObservations,
            8 => ProcessedDataType::EventProbability,
            192 => ProcessedDataType::Experimental,
            255 => ProcessedDataType::Missing,
            _ => ProcessedDataType::Unknown(value)
        };

        return Ok(ref_time_significance);
    }
}
