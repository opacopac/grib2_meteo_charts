use crate::grib2_section5::data_representation_template::DataRepresentationTemplate;
use crate::tests::test_common::read_test_layer;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section5.length;

    assert_eq!(21, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section5.section_number;

    assert_eq!(5, result);
}


#[test]
fn it_reads_the_correct_number_of_data_points() {
    let layer = read_test_layer();

    let result = layer.section5.data_points;

    assert_eq!(754862, result);
}


#[test]
fn it_reads_the_correct_data_representation_template() {
    let layer = read_test_layer();

    let result = layer.section5.data_representation_template;

    match result {
        DataRepresentationTemplate::GridPointDataSimplePacking(_tpl) => {},
        DataRepresentationTemplate::Missing => panic!("wrong data representation template: 65535"),
        DataRepresentationTemplate::Unknown(nr) => panic!("wrong data representation template: {}", nr)
    };
}
