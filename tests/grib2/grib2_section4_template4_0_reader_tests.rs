use meteo_grib2_renderer::grib2::section4::product_definition_template::ProductDefinitionTemplate;
use meteo_grib2_renderer::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

use crate::read_test_document;

fn read_tpl_4_0() -> ProductDefinitionTemplate4_0 {
    let layer = read_test_document();

    let result = layer.section4.product_definition_template;
    return match result {
        ProductDefinitionTemplate::Template4_0(tpl) => tpl,
        _ => panic!("wrong product defintion template")
    }
}


#[test]
fn it_reads_the_correct_parameter_category_and_number() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.parameter_category;
    assert_eq!(6, result);

    let result = tpl40.parameter_number;
    assert_eq!(199, result);
}


#[test]
fn it_reads_the_correct_generating_process_type_and_bg_identifier() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.generating_process_type;
    assert_eq!(2, result);

    let result = tpl40.generating_process_identifier;
    assert_eq!(0, result);

    let result = tpl40.generating_process;
    assert_eq!(11, result);
}
