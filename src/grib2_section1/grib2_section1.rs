use std::error::Error;
use chrono::NaiveDateTime;
use crate::grib2_section1::grib2_processed_data_type::Grib2ProcessedDataType;
use crate::grib2_section1::grib2_production_status::Grib2ProductionStatus;
use crate::grib2_section1::grib2_ref_time_significance::Grib2RefTimeSignificance;


pub struct Grib2Section1 {
    pub length: u32,
    pub section_number: u8,
    pub center: u16,
    pub subcenter: u16,
    pub master_table_version: u8,
    pub local_table_version: u8,
    pub ref_time_significance: Grib2RefTimeSignificance,
    pub ref_time: NaiveDateTime,
    pub production_status: Grib2ProductionStatus,
    pub processed_data_type: Grib2ProcessedDataType
}


impl Grib2Section1 {
    pub fn new(
        length: u32,
        section_number: u8,
        center: u16,
        subcenter: u16,
        master_table_version: u8,
        local_table_version: u8,
        ref_time_significance: Grib2RefTimeSignificance,
        ref_time: NaiveDateTime,
        production_status: Grib2ProductionStatus,
        processed_data_type: Grib2ProcessedDataType
    ) -> Result<Grib2Section1, Box<dyn Error>> {
        return Ok(Grib2Section1 {
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
        });
    }
}
