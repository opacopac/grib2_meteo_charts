use meteo_grib2_renderer::grib2::section4::product_definition_template::ProductDefinitionTemplate;

use crate::read_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_document();

    let result = layer.section4.length;

    assert_eq!(34, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_document();

    let result = layer.section4.section_number;

    assert_eq!(4, result);
}


#[test]
fn it_reads_the_correct_number_of_coordinate_values() {
    let layer = read_test_document();

    let result = layer.section4.coordinate_values;

    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_product_definition_template() {
    let layer = read_test_document();

    let result = layer.section4.product_definition_template;

    match result {
        ProductDefinitionTemplate::Template4_0(_tpl) => {},
        ProductDefinitionTemplate::Missing => panic!("wrong product definition template: 255"),
        ProductDefinitionTemplate::Unknown(nr) => panic!("wrong product definition template: {}", nr)
    };
}
