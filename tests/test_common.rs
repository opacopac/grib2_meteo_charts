use meteo_grib2_renderer::dwd::dwd_icon_d2_cloud_cover_layer::DwdIconD2CloudCoverLayer;
use meteo_grib2_renderer::grib2::common::grib2_error::Grib2Error;
use meteo_grib2_renderer::grib2::document::grib2_document::Grib2Document;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

mod grib2;
mod dwd;
mod meteo_chart;

pub const DATA_DIR: &str = "./tests/data/";
pub const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


pub fn read_test_document_result() -> Result<Grib2Document, Grib2Error> {
    return Grib2DocumentReader::read_file(CLCT_TEST_FILE);
}


pub fn read_test_document() -> Grib2Document {
    return read_test_document_result().unwrap();
}


pub fn read_test_cloud_cover_layer() -> DwdIconD2CloudCoverLayer {
    let doc = read_test_document();
    let ccl = DwdIconD2CloudCoverLayer::from_grib2(doc).unwrap();

    return ccl;
}
