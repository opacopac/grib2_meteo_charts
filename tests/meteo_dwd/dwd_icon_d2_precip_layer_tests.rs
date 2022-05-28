use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_precip_layer::DwdPrecipLayer;
use meteo_grib2_renderer::meteo_dwd::regular_grid_converter::RegularGridConverter;

use crate::CLCT_ICON_D2_TEST_FILE;

pub const PREC_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042500_001_2d_tot_prec.grib2";


#[test]
fn it_successfully_reads_a_precip_test_file() {
    let doc = Grib2DocumentReader::read_file(PREC_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = DwdPrecipLayer::new(grid);

    assert!(true);
}


#[test]
#[should_panic]
fn it_returns_an_error_for_a_non_precip_test_file() {
    let doc = Grib2DocumentReader::read_file(CLCT_ICON_D2_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();
    let _layer = DwdPrecipLayer::new(grid);
}
