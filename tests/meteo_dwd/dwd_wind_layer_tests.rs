use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_wind_layer::DwdWindLayer;

pub const WIND_U_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
pub const WIND_V_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";
pub const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";
pub const WIND_V_EU_TEST_FILE: &str = "./tests/data/icon-eu_europe_regular-lat-lon_single-level_2022050700_000_V_10M.grib2";

#[test]
fn it_successfully_creates_a_wind_test_file_from_wind_u_and_v_grib_docs() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();

    let result = DwdWindLayer::from_grib2(doc_u, doc_v);

    assert!(result.is_ok());
}


#[test]
fn it_returns_an_error_for_a_non_wind_file() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();

    let result = DwdWindLayer::from_grib2(doc_u, doc_v);

    assert!(result.is_err());
}


#[test]
fn it_returns_an_error_when_u_and_v_are_mixed_up() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();

    let result = DwdWindLayer::from_grib2(doc_v, doc_u);

    assert!(result.is_err());
}


#[test]
fn it_returns_an_error_when_the_grid_sizes_dont_match() {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_EU_TEST_FILE).unwrap();

    let result = DwdWindLayer::from_grib2(doc_u, doc_v);

    assert!(result.is_err());
}
