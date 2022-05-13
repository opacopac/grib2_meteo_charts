use meteo_grib2_renderer::grib2::common::grib2_error::Grib2Error;
use meteo_grib2_renderer::grib2::document::grib2_document::Grib2Document;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_icon_d2_tot_cloud_cover_layer::DwdIconD2TotalCloudCoverLayer;

mod grib2;
mod meteo_dwd;
mod chart;

pub const DATA_DIR: &str = "./tests/data/";
pub const CLCT_ICON_D2_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";
pub const CLCT_ICON_GLOBAL_TEST_FILE: &str = "./tests/data/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";


pub fn read_icon_d2_test_document_result() -> Result<Grib2Document, Grib2Error> {
    return Grib2DocumentReader::read_file(CLCT_ICON_D2_TEST_FILE);
}


pub fn read_icon_d2_test_document() -> Grib2Document {
    return read_icon_d2_test_document_result().unwrap();
}


pub fn read_icon_global_test_document() -> Grib2Document {
    let result = Grib2DocumentReader::read_file(CLCT_ICON_GLOBAL_TEST_FILE);

    return result.unwrap();
}


pub fn read_test_cloud_cover_layer() -> DwdIconD2TotalCloudCoverLayer {
    let doc = read_icon_d2_test_document();
    let ccl = DwdIconD2TotalCloudCoverLayer::from_grib2(doc).unwrap();

    return ccl;
}
