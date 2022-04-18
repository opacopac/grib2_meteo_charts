use std::error::Error;
use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;
use crate::Grib2CloudCoverReader;

pub const DATA_DIR: &str = "./src/tests/data/";
pub const CLCT_TEST_FILE: &str = "./src/tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


pub fn read_test_layer_result() -> Result<Grib2CloudCoverLayer, Box<dyn Error>> {
    return Grib2CloudCoverReader::read_file(CLCT_TEST_FILE);
}


pub fn read_test_layer() -> Grib2CloudCoverLayer {
    return read_test_layer_result().unwrap();
}
