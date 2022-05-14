use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_global_tot_cloud_cover_layer::DwdIconGlobalTotalCloudCoverLayer;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let result = DwdIconGlobalTotalCloudCoverLayer::from_grib2(doc);
    assert!(result.is_ok());

    let layer = result.unwrap();
    assert_eq!(MeteoParameterCategory::Cloud, layer.parameter_category);
    assert_eq!(199, layer.parameter_number);

    // TODO: grid
}
