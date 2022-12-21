use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::regular_grid_converter::RegularGridConverter;

pub const HHL_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_time-invariant_2022122100_000_66_hhl.grib2";


#[test]
fn it_successfully_reads_a_hhl_test_file() {
    let doc = Grib2DocumentReader::read_file(HHL_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = DwdCloudLayer::new(grid);

    assert!(true);
}
