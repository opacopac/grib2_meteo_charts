use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::regular_grid_converter::RegularGridConverter;

pub const CLC_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_model-level_2022122115_000_25_clc.grib2";


#[test]
fn it_successfully_reads_a_d2_clc_test_file() {
    let doc = Grib2DocumentReader::read_file(CLC_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let layer = DwdCloudLayer::new(grid);

    let pos1 = LatLon { lat: 47.0, lon: 7.0 };
    let clc_value1 = layer.get_cloud_cover_by_lat_lon(&pos1).unwrap();
    assert_eq!(0.0, clc_value1);

    let pos2 = LatLon { lat: 49.82, lon: 3.14 };
    let clc_value2 = layer.get_cloud_cover_by_lat_lon(&pos2).unwrap();
    assert_eq!(100.0, clc_value2);
}
