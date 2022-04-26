use meteo_grib2_renderer::dwd::dwd_zonal_wind_layer::DwdZonalWindLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grib2::section4::meteo_parameter_category::MeteoParameterCategory;

use crate::CLCT_TEST_FILE;

pub const PREC_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";


#[test]
fn it_successfully_reads_a_zonal_wind_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();

    let result = DwdZonalWindLayer::from_grib2(doc);
    assert!(result.is_ok());

    let layer = result.unwrap();
    assert_eq!(MeteoParameterCategory::Momentum, layer.parameter_category);
    assert_eq!(2, layer.parameter_number);
}


#[test]
fn it_returns_an_error_for_a_non_zonal_wind_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let layer = DwdZonalWindLayer::from_grib2(doc);

    assert!(layer.is_err());
}
