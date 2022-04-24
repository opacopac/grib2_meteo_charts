use meteo_grib2_renderer::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use meteo_grib2_renderer::grib2::section3::grid_definition_template_3_0::GridDefinitionTemplate3_0;
use meteo_grib2_renderer::grib2::section3::shape_of_earth::ShapeOfEarth;

use crate::read_test_document;

fn read_tpl_3_0() -> GridDefinitionTemplate3_0 {
    let layer = read_test_document();

    let result = layer.section3.grid_definition_template;
    return match result {
        GridDefinitionTemplate::LatitudeLongitude(tpl) => tpl,
        _ => panic!("wrong grid defintion template")
    }
}


#[test]
fn it_reads_the_correct_shape_of_earth() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.shape_of_earth;

    assert_eq!(ShapeOfEarth::SphericalRadius6371229, result);
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

    assert_eq!(43.180000, tpl30.first_grid_point_lat);
    assert_eq!(356.060000, tpl30.first_grid_point_lon);
}


#[test]
fn it_reads_the_correct_resolution_and_component_flags() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.resolution_component_flags;
    assert_eq!(true, result.i_direction_increments_not_given);
    assert_eq!(true, result.j_direction_increments_not_given);
    assert_eq!(false, result.u_v_relative_to_e_n);
}


#[test]
fn it_reads_the_correct_last_grid_point_lat_lon() {
    let tpl30 = read_tpl_3_0();

    assert_eq!(58.080000, tpl30.last_grid_point_lat);
    assert_eq!(20.340000, tpl30.last_grid_point_lon);
}


#[test]
fn it_reads_the_correct_i_and_j_direction_increments() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.i_direction_increment;
    assert_eq!(0.020000, result);

    let result = tpl30.j_direction_increment;
    assert_eq!(0.020000, result);
}


#[test]
fn it_reads_the_correct_scanning_mode_flags() {
    let tpl30 = read_tpl_3_0();

    let result = tpl30.scanning_mode_flags;
    assert_eq!(true, result.scan_direction_first_row_i_is_positive);
    assert_eq!(true, result.scan_direction_first_row_j_is_negative);
    assert_eq!(true, result.adjacent_points_in_i_direction_consecutive);
    assert_eq!(true, result.all_rows_same_scan_direction);
    assert_eq!(true, result.odd_rows_offset_in_i_direction);
    assert_eq!(true, result.even_rows_offset_in_i_direction);
    assert_eq!(false, result.points_not_offset_in_j_direction);
    assert_eq!(true, result.rows_have_ni_points_cols_have_nj_points);
}
