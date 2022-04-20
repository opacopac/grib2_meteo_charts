use meteo_grib2_renderer::grib2::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use meteo_grib2_renderer::grib2::cloud_cover::cloud_cover_reader::CloudCoverReader;

use crate::test_common::{DATA_DIR, read_test_layer, read_test_layer_result};

mod test_common;

#[test]
fn it_reads_an_existing_grib2_file() {
    let result = read_test_layer_result();

    assert_eq!(false, result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let grib2_file =  DATA_DIR.to_string() + "notfound.grib2";

    let result = CloudCoverReader::read_file(&grib2_file);

    assert_eq!(true, result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_grib2_format() {
    let grib2_file = DATA_DIR.to_string() + "not_a_grib2_file.grib2";

    let result = CloudCoverReader::read_file(&grib2_file);

    assert_eq!(true, result.is_err());
}


#[test]
fn it_returns_the_value_some_data_points() {
    let layer = read_test_layer();

    let result1 = layer.get_value(0);
    assert_eq!(0.0, result1);

    let result2 = layer.get_value(912);
    assert_eq!(0.6885376, result2);
}
