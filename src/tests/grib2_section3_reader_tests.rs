use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;
use crate::grib2_section3::grib2_grid_definition_template::Grib2GridDefinitionTemplate;
use crate::grib2_section3::grib2_optional_point_interpretation::Grib2OptionalPointInterpretation;
use crate::tests::test_common::read_test_layer;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section3.length;

    assert_eq!(72, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section3.section_number;

    assert_eq!(3, result);
}


#[test]
fn it_reads_the_correct_grid_definition_source() {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_source;

    assert_eq!(Grib2GridDefinitionSource::GridDefinitionTemplate, result);
}


#[test]
fn it_reads_the_correct_number_of_datapoints() {
    let layer = read_test_layer();

    let result = layer.section3.number_of_datapoints;

    assert_eq!(906390, result);
}


#[test]
fn it_reads_the_correct_length_of_opt_points() {
    let layer = read_test_layer();

    let result = layer.section3.optional_point_length;

    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_optional_point_interpretation() {
    let layer = read_test_layer();

    let result = layer.section3.optional_point_interpretation;

    assert_eq!(Grib2OptionalPointInterpretation::None, result);
}


#[test]
fn it_reads_the_correct_grid_definition_template() {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_template;

    match result {
        Grib2GridDefinitionTemplate::LatLon(_tpl) => {},
        Grib2GridDefinitionTemplate::Missing => panic!("wrong grid definition template: 255"),
        Grib2GridDefinitionTemplate::Unknown(nr) => panic!("wrong grid definition template: {}", nr)
    };
}
