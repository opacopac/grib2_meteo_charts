use meteo_grib2_renderer::dwd::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use meteo_grib2_renderer::grib2::common::lat_lon::LatLon;

use crate::read_test_cloud_cover_layer;

#[test]
fn it_returns_the_value_of_data_points_by_index() {
    let layer = read_test_cloud_cover_layer();

    let result1 = layer.get_value_by_index(0);
    assert_eq!(CloudCoverLayer::MISSING_VALUE, result1);

    let result2 = layer.get_value_by_index(208);
    assert_eq!(0.5387573, result2);
}


#[test]
fn it_returns_the_correct_index_by_exact_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // first point
    let result1 = layer.get_index_by_lat_lon(&LatLon::new(43.18, 356.06));
    assert_eq!(0, result1);

    // second point
    let result1 = layer.get_index_by_lat_lon(&LatLon::new(43.18, 356.06 + 0.02));
    assert_eq!(1, result1);

    // first point in second row
    let result1 = layer.get_index_by_lat_lon(&LatLon::new(43.18 + 0.02, 356.06));
    assert_eq!(1215, result1);

    // last point
    let result2 = layer.get_index_by_lat_lon(&LatLon::new(58.08, 20.34));
    assert_eq!(1215 * 746 - 1, result2);
}


#[test]
fn it_returns_the_correct_index_by_approximate_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // near first point
    let result1 = layer.get_index_by_lat_lon(&LatLon::new(43.1801, 356.0601));
    assert_eq!(0, result1);

    // near last point
    let result2 = layer.get_index_by_lat_lon(&LatLon::new(58.0801, 20.3401));
    assert_eq!(1215 * 746 - 1, result2);

    // near middle point (50.64, 8.20)
    let result3 = layer.get_index_by_lat_lon(&LatLon::new(43.18 + 373.0 * 0.02, (356.06 + 607.0 * 0.02) % 360.0));
    assert_eq!(373 * 1215 + 607, result3);
}


#[test]
fn it_returns_the_value_of_data_points_by_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    // first point value
    let result1 = layer.get_value_by_lat_lon(&LatLon::new(43.18, 356.06));
    assert_eq!(CloudCoverLayer::MISSING_VALUE, result1);

    // last point value
    let result2 = layer.get_value_by_lat_lon(&LatLon::new(58.08, 20.34));
    assert_eq!(CloudCoverLayer::MISSING_VALUE, result2);

    // middle point value
    let result3 = layer.get_value_by_lat_lon(&LatLon::new(43.18, 0.22));
    assert_eq!(0.5387573, result3);

    // middle point value
    let result3 = layer.get_value_by_lat_lon(&LatLon::new(50.64, 8.20));
    assert_eq!(0.0, result3);
}
