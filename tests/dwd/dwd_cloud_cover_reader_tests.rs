use meteo_grib2_renderer::grib2::common::lat_lon::LatLon;

use crate::read_test_cloud_cover_layer;

#[test]
fn it_returns_the_value_of_data_points_by_index() {
    let layer = read_test_cloud_cover_layer();

    let result1 = layer.get_value_by_index(0);
    assert_eq!(0.0, result1);

    let result2 = layer.get_value_by_index(912);
    assert_eq!(0.6885376, result2);
}


#[test]
fn it_returns_the_value_of_data_points_by_lat_lon() {
    let layer = read_test_cloud_cover_layer();

    let result1 = layer.get_value_by_lat_lon(LatLon::new(43.18, 356.06));
    assert_eq!(0.0, result1);

    /*let result2 = layer.get_value_by_lat_lon(LatLon::new(50.00, 359.00));
    assert_eq!(99.0, result2);*/
}
