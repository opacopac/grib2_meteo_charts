use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;
use crate::grib2_section3::grib2_grid_definition_template::Grib2GridDefinitionTemplate;
use crate::grib2_section3::grib2_grid_definition_template_type::Grib2GridDefinitionTemplateType;
use crate::grib2_section3::grib2_optional_point_interpretation::Grib2OptionalPointInterpretation;
use crate::tests::test_common::read_test_layer;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section3.length;

    assert_eq!(35, result);
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

    assert_eq!(2949120, result);
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
fn it_reads_the_correct_grid_definition_template_type() {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_template_type;

    assert_eq!(Grib2GridDefinitionTemplateType::UnstructuredGrid, result);
}


/*#[test]
fn it_reads_the_correct_grid_definition_template() {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_template;

    assert_eq!(Grib2GridDefinitionTemplate::UnstructuredGrid, result);
}
*/
