use meteo_grib2_renderer::dwd::dwd_precip_layer::PrecipLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

use crate::CLCT_TEST_FILE;

pub const PREC_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042500_001_2d_tot_prec.grib2";


#[test]
fn it_successfully_reads_a_precip_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();

    let layer = PrecipLayer::new(doc);

    assert!(layer.is_ok());
}


#[test]
fn it_returns_an_error_for_a_cloud_cover_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let layer = PrecipLayer::new(doc);

    assert!(layer.is_err());
}
