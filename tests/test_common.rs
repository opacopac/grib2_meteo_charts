use meteo_grib2_renderer::grib2::common::grib2_error::Grib2Error;
use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::document::grib2_document::Grib2Document;

mod grib2;
mod dwd;
mod meteo_chart;
mod meteo_swiss;
mod netcdf;

pub const DATA_DIR: &str = "./tests/resources/";
pub const CLCT_ICON_D2_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";
pub const CLCT_ICON_GLOBAL_TEST_FILE: &str = "./tests/resources/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";


pub fn read_icon_d2_test_document_result() -> Result<Grib2Document, Grib2Error> {
    FileToGridConverter::read_single_doc_from_file_or_url(CLCT_ICON_D2_TEST_FILE)
}
