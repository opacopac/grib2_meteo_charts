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
fn it_reads_the_correct_spherical_earth_radius_scale_factor_and_value() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.spherical_earth_radius;
    assert_eq!(255, result.factor);
    assert_eq!(4294967295, result.value);
}


#[test]
fn it_reads_the_correct_oblated_spheroid_earth_major_axis_scale_factor_and_value() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.oblated_spheroid_earth_major_axis;
    assert_eq!(255, result.factor);
    assert_eq!(4294967295, result.value);
}


#[test]
fn it_reads_the_correct_oblated_spheroid_earth_minor_axis_scale_factor_and_value() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.oblated_spheroid_earth_minor_axis;
    assert_eq!(255, result.factor);
    assert_eq!(4294967295, result.value);
}


#[test]
fn it_reads_the_correct_number_of_points_along_parallel_and_meridian() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.number_of_points_along_parallel;
    assert_eq!(1215, result);

    let result = tpl30.number_of_points_along_meridian;
    assert_eq!(746, result);
}


#[test]
fn it_reads_the_correct_initial_production_domain_basic_angle_and_subdivision() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.initial_production_domain_basic_angle;
    assert_eq!(0, result);

    let result = tpl30.initial_production_domain_subdivision;
    assert_eq!(4294967295, result);
}


#[test]
fn it_reads_the_correct_first_grid_point_lat_lon() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.first_grid_point;
    assert_eq!(43.180000, result.lat);
    assert_eq!(356.060000, result.lon);
}


#[test]
fn it_reads_the_correct_resolution_and_component_flags() {
    let tpl30 = read_tpl_3_0();

    let flags = tpl30.resolution_component_flags;
    let result = flags.has_i_direction_increments;
    assert_eq!(false, result);

    let result = flags.has_j_direction_increments;
    assert_eq!(false, result);

    let result = flags.u_v_relative_to_e_n;
    assert_eq!(true, result);
}


#[test]
fn it_reads_the_correct_last_grid_point_lat_lon() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.last_grid_point;
    assert_eq!(58.080000, result.lat);
    assert_eq!(20.340000, result.lon);
}


#[test]
fn it_reads_the_correct_i_and_j_direction_increments() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.i_direction_increment;
    assert_eq!(0.020000, result);

    let result = tpl30.j_direction_increment;
    assert_eq!(0.020000, result);
}


