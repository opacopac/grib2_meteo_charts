use meteo_grib2_renderer::grib2::section3::grid_definition_source::GridDefinitionSource;
use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use meteo_grib2_renderer::grib2::section3::optional_point_interpretation::OptionalPointInterpretation;

use crate::read_icon_d2_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.length;

    assert_eq!(72, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.section_number;

    assert_eq!(3, result);
}


#[test]
fn it_reads_the_correct_grid_definition_source() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.grid_definition_source;

    assert_eq!(GridDefinitionSource::GridDefinitionTemplate, result);
}


#[test]
fn it_reads_the_correct_number_of_datapoints() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.number_of_datapoints;

    assert_eq!(906390, result);
}


#[test]
fn it_reads_the_correct_length_of_opt_points() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.optional_point_length;

    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_optional_point_interpretation() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.optional_point_interpretation;

    assert_eq!(OptionalPointInterpretation::None, result);
}


#[test]
fn it_reads_the_correct_grid_definition_template() {
    let layer = read_icon_d2_test_document();

    let result = layer.section3.grid_definition_template;

    match result {
        GridDefinitionTemplate::LatitudeLongitude(_tpl) => {},
        GridDefinitionTemplate::Missing => panic!("wrong grid definition template: 255"),
        GridDefinitionTemplate::Unknown(nr) => panic!("wrong grid definition template: {}", nr)
    };
}
