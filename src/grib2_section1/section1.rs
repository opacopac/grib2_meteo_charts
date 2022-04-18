use std::error::Error;
use chrono::NaiveDateTime;
use crate::grib2_section1::processed_data_type::ProcessedDataType;
use crate::grib2_section1::production_status::ProductionStatus;
use crate::grib2_section1::ref_time_significance::RefTimeSignificance;


pub struct Section1 {
    pub length: u32,
    pub section_number: u8,
    pub center: u16,
    pub subcenter: u16,
    pub master_table_version: u8,
    pub local_table_version: u8,
    pub ref_time_significance: RefTimeSignificance,
    pub ref_time: NaiveDateTime,
    pub production_status: ProductionStatus,
    pub processed_data_type: ProcessedDataType
}


impl Section1 {
    pub fn new(
        length: u32,
        section_number: u8,
        center: u16,
        subcenter: u16,
        master_table_version: u8,
        local_table_version: u8,
        ref_time_significance: RefTimeSignificance,
        ref_time: NaiveDateTime,
        production_status: ProductionStatus,
        processed_data_type: ProcessedDataType
    ) -> Result<Section1, Box<dyn Error>> {
        return Ok(Section1 {
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
