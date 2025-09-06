use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::meteo_layer::meteo_cloud_layer::MeteoCloudLayer;

pub const CLC_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_model-level_2022122115_000_25_clc.grib2";
pub const CLC_TEST_FILE_NO_DATAPOINTS: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_model-level_2023021318_002_30_clc.grib2";


#[test]
fn it_successfully_reads_a_d2_clc_test_file() {
    let doc = FileToGridConverter::read_single_doc_from_file_or_url(CLC_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = MeteoCloudLayer::new(grid);

    let pos1 = LatLon { lat: 47.0, lon: 7.0 };
    let clc_value1 = layer.get_cloud_cover_by_lat_lon(&pos1).unwrap();
    assert_eq!(0.0, clc_value1);

    let pos2 = LatLon { lat: 49.82, lon: 3.14 };
    let clc_value2 = layer.get_cloud_cover_by_lat_lon(&pos2).unwrap();
    assert_eq!(100.0, clc_value2);
}


#[test]
fn it_successfully_reads_a_d2_clc_test_file_without_datapoints() {
    let doc = FileToGridConverter::read_single_doc_from_file_or_url(CLC_TEST_FILE_NO_DATAPOINTS).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = MeteoCloudLayer::new(grid);

    let pos1 = LatLon { lat: 47.0, lon: 7.0 };
    let clc_value1 = layer.get_cloud_cover_by_lat_lon(&pos1);
    assert_eq!(None, clc_value1);
}
