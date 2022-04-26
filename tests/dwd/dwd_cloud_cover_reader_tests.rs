use meteo_grib2_renderer::dwd::dwd_cloud_cover_layer::DwdCloudCoverLayer;
use meteo_grib2_renderer::dwd::value_grid::ValueGrid;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

use crate::dwd::dwd_precip_reader_tests::PREC_TEST_FILE;
use crate::read_test_cloud_cover_layer;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


#[test]
fn it_successfully_reads_a_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let layer = DwdCloudCoverLayer::from_grib2(doc);

    assert!(layer.is_ok());
}


#[test]
fn it_returns_an_error_for_a_non_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();

    let layer = DwdCloudCoverLayer::from_grib2(doc);

    assert!(layer.is_err());
}


#[test]
fn it_returns_the_correct_grid_parameters() {
    let layer = read_test_cloud_cover_layer();

    let result1 = layer.value_grid.grid.start_pos;
    assert_eq!(43.18, result1.lat);
    assert_eq!(356.06 - 360.0, result1.lon);

    let result1 = layer.value_grid.grid.end_pos;
    assert_eq!(58.08, result1.lat);
    assert_eq!(20.34, result1.lon);

    let result1 = layer.value_grid.grid.lat_inc_deg;
    assert_eq!(0.02, result1);

    let result1 = layer.value_grid.grid.lon_inc_deg;
    assert_eq!(0.02, result1);

    let result1 = layer.value_grid.grid.lat_grid_points;
    assert_eq!(746, result1);

    let result1 = layer.value_grid.grid.lon_grid_points;
    assert_eq!(1215, result1);
}


#[test]
fn it_returns_the_value_of_data_points_by_index() {
    let layer = read_test_cloud_cover_layer();

    let result1 = layer.value_grid.get_value_by_index(0);
    assert_eq!(ValueGrid::MISSING_VALUE, result1);

    let result2 = layer.value_grid.get_value_by_index(208);
    assert_eq!(0.5387573, result2);
}


#[test]
fn it_returns_the_correct_index_by_exact_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // first point
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18, 356.06));
    assert_eq!(0, result1);

    // second point
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18, 356.06 + 0.02));
    assert_eq!(1, result1);

    // first point in second row
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18 + 0.02, 356.06));
    assert_eq!(1215, result1);

    // last point
    let result2 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(58.08, 20.34));
    assert_eq!(1215 * 746 - 1, result2);
}


#[test]
fn it_returns_the_correct_index_by_approximate_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // near first point
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18 + 0.001, 356.06 + 0.001));
    assert_eq!(0, result1);

    // near last point
    let result2 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(58.08 - 0.001, 20.34 - 0.001));
    assert_eq!(1215 * 746 - 1, result2);

    // near middle point (50.64, 8.20)
    let result3 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18 + 373.0 * 0.02, (356.06 + 607.0 * 0.02) % 360.0));
    assert_eq!(373 * 1215 + 607, result3);
}


#[test]
fn it_returns_the_correct_index_by_negative_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // first point
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18, -3.94));
    assert_eq!(0, result1);

    // second first point
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(43.18, -3.94 + 0.02));
    assert_eq!(1, result1);
}


#[test]
fn it_returns_an_out_of_bounds_index_when_outside_of_lat_lon_extent() {
    let layer = read_test_cloud_cover_layer();
    let out_of_bounds_index = 1215 * 746;

    // below extent
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(20.0, 10.0));
    assert_eq!(out_of_bounds_index, result1);

    // above extent
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(60.0, 10.0));
    assert_eq!(out_of_bounds_index, result1);

    // east of extent
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(50.0, 350.0));
    assert_eq!(out_of_bounds_index, result1);

    // west of extent
    let result1 = layer.value_grid.get_index_by_lat_lon(&LatLon::new(50.0, 30.0));
    assert_eq!(out_of_bounds_index, result1);
}


#[test]
fn it_returns_the_value_of_data_points_by_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // first point value
    let result1 = layer.value_grid.get_value_by_lat_lon(&LatLon::new(43.18, 356.06));
    assert_eq!(ValueGrid::MISSING_VALUE, result1);

    // last point value
    let result2 = layer.value_grid.get_value_by_lat_lon(&LatLon::new(58.08, 20.34));
    assert_eq!(ValueGrid::MISSING_VALUE, result2);

    // middle point value
    let result3 = layer.value_grid.get_value_by_lat_lon(&LatLon::new(43.18, 0.22));
    assert_eq!(0.5387573, result3);

    // middle point value
    let result3 = layer.value_grid.get_value_by_lat_lon(&LatLon::new(50.64, 8.20));
    assert_eq!(0.0, result3);
}
