use crate::grib2_section3::grib2_grid_definition_template::Grib2GridDefinitionTemplate;
use crate::grib2_section3::grib2_grid_definition_template_3_0::Grib2gridDefinitionTemplate3_0;
use crate::grib2_section3::grib2_shape_of_earth::Grib2ShapeOfEarth;
use crate::tests::test_common::read_test_layer;


fn read_tpl_3_0() -> Grib2gridDefinitionTemplate3_0 {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_template;
    return match result {
        Grib2GridDefinitionTemplate::LatLon(tpl) => tpl,
        _ => panic!("wrong grid defintion template")
    }
}


#[test]
fn it_reads_the_correct_shape_of_earth() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.shape_of_earth;

    assert_eq!(Grib2ShapeOfEarth::SphericalRadius6371229, result);
}


#[test]
fn it_reads_the_correct_spherical_earth_radius_scale_factor() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.spherical_earth_radius_scale_factor;

    assert_eq!(255, result);
}


#[test]
fn it_reads_the_correct_spherical_earth_radius_scale_value() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.spherical_earth_radius_scale_value;

    assert_eq!(255, result);
}
