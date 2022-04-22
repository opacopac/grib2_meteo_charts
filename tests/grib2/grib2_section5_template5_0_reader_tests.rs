use meteo_grib2_renderer::grib2::section5::data_representation_template::DataRepresentationTemplate;
use meteo_grib2_renderer::grib2::section5::data_representation_template_5_0::DataRepresentationTemplate5_0;
use meteo_grib2_renderer::grib2::section5::original_field_type::OriginalFieldType;

use crate::read_test_document;

fn read_tpl_5_0() -> DataRepresentationTemplate5_0 {
    let layer = read_test_document();

    let result = layer.section5.data_representation_template;
    return match result {
        DataRepresentationTemplate::GridPointDataSimplePacking(tpl) => tpl,
        _ => panic!("wrong grid defintion template")
    }
}


#[test]
fn it_reads_the_correct_reference_value() {
    let tpl = read_tpl_5_0();

    let result = tpl.reference_value;

    assert_eq!(0.0, result);
}


#[test]
fn it_reads_the_correct_binary_scale_factor() {
    let tpl = read_tpl_5_0();

    let result = tpl.binary_scale_factor_e;

    assert_eq!(-15, result);
}


#[test]
fn it_reads_the_correct_decimal_scale_factor() {
    let tpl = read_tpl_5_0();

    let result = tpl.decimal_scale_factor_d;

    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_number_of_bits() {
    let tpl = read_tpl_5_0();

    let result = tpl.number_of_bits;

    assert_eq!(16, result);
}


#[test]
fn it_reads_the_correct_original_field_type() {
    let tpl = read_tpl_5_0();

    let result = tpl.original_field_type;

    assert_eq!(OriginalFieldType::FloatingPoint, result);
}
