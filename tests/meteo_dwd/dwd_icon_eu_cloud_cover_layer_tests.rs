use meteo_grib2_renderer::meteo_dwd::dwd_icon_d2_tot_cloud_cover_layer::DwdIconD2TotalCloudCoverLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

use crate::meteo_dwd::dwd_precip_layer_tests::PREC_TEST_FILE;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon-eu_europe_regular-lat-lon_single-level_2022042700_047_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_eu_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let result = DwdIconD2TotalCloudCoverLayer::from_grib2(doc);
    assert!(result.is_ok());

    let layer = result.unwrap();
    assert_eq!(MeteoParameterCategory::Cloud, layer.parameter_category);
    assert_eq!(199, layer.parameter_number);
}


#[test]
fn it_returns_an_error_for_a_non_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();

    let layer = DwdIconD2TotalCloudCoverLayer::from_grib2(doc);

    assert!(layer.is_err());
}
