use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::section1::processed_data_type::ProcessedDataType;
use crate::grib2::section1::production_status::ProductionStatus;
use crate::grib2::section1::ref_time_significance::RefTimeSignificance;
use chrono::NaiveDateTime;


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
    pub processed_data_type: ProcessedDataType,
}


const SECTION_NUMBER: u8 = 1;


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
        processed_data_type: ProcessedDataType,
    ) -> Result<Section1, Grib2Error> {
        if section_number != SECTION_NUMBER {
            return Err(Grib2Error::InvalidData(
                format!("invalid section number '{}', expected: {}", section_number, SECTION_NUMBER)
            ));
        }

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
            processed_data_type,
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::grib2::section1::processed_data_type::ProcessedDataType;
    use crate::grib2::section1::production_status::ProductionStatus;
    use crate::grib2::section1::ref_time_significance::RefTimeSignificance;
    use crate::grib2::section1::section1::Section1;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};


    #[test]
    fn it_detects_an_incorrect_section_number() {
        let result = Section1::new(
            0,
            0,
            0,
            0,
            0,
            0,
            RefTimeSignificance::Missing,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 04, 18).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            ProductionStatus::Missing,
            ProcessedDataType::Missing,
        );

        assert!(result.is_err());
    }
}
