use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_eu_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = DwdCloudLayer::new(grid);

    assert!(true);
}
