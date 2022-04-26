use meteo_grib2_renderer::dwd::dwd_cloud_cover_layer::DwdCloudCoverLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

use crate::dwd::dwd_precip_reader_tests::PREC_TEST_FILE;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


#[test]
fn it_successfully_reads_a_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let layer = DwdCloudCoverLayer::from_grib2(doc);

    assert!(layer.is_ok());
}


#[test]
fn it_returns_an_error_for_a_non_clct_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();

    let layer = DwdCloudCoverLayer::from_grib2(doc);

    assert!(layer.is_err());
}
