use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::regular_grid_converter::RegularGridConverter;

pub const CLCT_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


#[test]
fn it_successfully_reads_a_d2_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = DwdCloudLayer::new(grid);

    assert!(true);
}
