use meteo_grib2_renderer::grib2_cloud_cover::cloud_cover_reader::CloudCoverReader;

const CLCT_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";

fn main() {
    println!("Hello, world!");
    let _layer = CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();
}
