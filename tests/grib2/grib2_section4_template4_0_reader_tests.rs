use meteo_grib2_renderer::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use meteo_grib2_renderer::grib2::section4::product_definition_template::ProductDefinitionTemplate;
use meteo_grib2_renderer::grib2::section4::product_definition_template_4_0::ProductDefinitionTemplate4_0;

use crate::read_test_document;

fn read_tpl_4_0() -> ProductDefinitionTemplate4_0 {
    let layer = read_test_document();

    let result = layer.section4.product_definition_template;
    return match result {
        ProductDefinitionTemplate::Template4_0(tpl) => tpl,
        _ => panic!("wrong product defintion template '{:?}'", result)
    }
}


#[test]
fn it_reads_the_correct_parameter_category_and_number() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.parameter_category;

    match result {
        MeteoParameterCategory::Cloud => {},
        _ => panic!("wrong meteo parameter category '{:?}'", result),
    };

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


#[test]
fn it_reads_the_correct_hours_min_cutoff_after_ref_time() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.hours_cutoff;
    assert_eq!(0, result);

    let result = tpl40.mins_cutoff;
    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_time_units_and_value() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.forecast_time_unit;
    assert_eq!(0, result);

    let result = tpl40.forecast_time_value;
    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_first_fixed_surface_parameters() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.fixed_surface1_type;
    assert_eq!(0, result);

    let result = tpl40.fixed_surface1_scale_factor_value;
    assert_eq!(0, result.factor);
    assert_eq!(65536, result.value);
}


#[test]
fn it_reads_the_correct_second_fixed_surface_parameters() {
    let tpl40 = read_tpl_4_0();

    let result = tpl40.fixed_surface2_type;
    assert_eq!(0, result);

    let result = tpl40.fixed_surface2_scale_factor_value;
    assert_eq!(0, result.factor);
    assert_eq!(16777215, result.value);
}