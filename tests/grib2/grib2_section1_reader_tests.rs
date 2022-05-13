use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use meteo_grib2_renderer::grib2::section1::processed_data_type::ProcessedDataType;
use meteo_grib2_renderer::grib2::section1::production_status::ProductionStatus;
use meteo_grib2_renderer::grib2::section1::ref_time_significance::RefTimeSignificance;

use crate::read_icon_d2_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_icon_d2_test_document();

    let result = layer.section1.length;

    assert_eq!(21, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_icon_d2_test_document();

    let result = layer.section1.section_number;

    assert_eq!(1, result);
}


#[test]
fn it_reads_the_correct_center_and_subcenter() {
    let layer = read_icon_d2_test_document();

    let result1 = layer.section1.center;
    let result2 = layer.section1.subcenter;

    assert_eq!(78, result1); // Offenbach
    assert_eq!(255, result2); // none
}


#[test]
fn it_reads_the_correct_grib_master_and_sub_table_numbers() {
    let layer = read_icon_d2_test_document();

    let result1 = layer.section1.master_table_version;
    let result2 = layer.section1.local_table_version;

    assert_eq!(19, result1);
    assert_eq!(1, result2);
}


#[test]
fn it_reads_the_correct_reference_time() {
    let layer = read_icon_d2_test_document();

    let result1 = layer.section1.ref_time_significance;
    let result2 = layer.section1.ref_time;
    let expected_time = NaiveDateTime::new(
        NaiveDate::from_ymd(2022, 4, 17),
        NaiveTime::from_hms(0, 0, 0)
    );

    assert_eq!(RefTimeSignificance::StartOfForecast, result1);
    assert_eq!(expected_time, result2);
}


#[test]
fn it_reads_the_correct_production_status() {
    let layer = read_icon_d2_test_document();

    let result = layer.section1.production_status;

    assert_eq!(ProductionStatus::Operational, result);
}


#[test]
fn it_reads_the_correct_type_of_data() {
    let layer = read_icon_d2_test_document();

    let result = layer.section1.processed_data_type;

    assert_eq!(ProcessedDataType::Forecast, result);
}
