use std::error::Error;
use crate::grib2_cloud_cover::cloud_cover_layer::CloudCoverLayer;
use crate::CloudCoverReader;

pub const DATA_DIR: &str = "./src/tests/data/";
pub const CLCT_TEST_FILE: &str = "./src/tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


pub fn read_test_layer_result() -> Result<CloudCoverLayer, Box<dyn Error>> {
    return CloudCoverReader::read_file(CLCT_TEST_FILE);
}


pub fn read_test_layer() -> CloudCoverLayer {
    return read_test_layer_result().unwrap();
}
