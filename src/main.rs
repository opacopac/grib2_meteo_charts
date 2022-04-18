use crate::grib2_cloud_cover::cloud_cover_reader::CloudCoverReader;

mod grib2_common;
mod grib2_cloud_cover;
mod grib2_section0;
mod grib2_section1;
mod grib2_section2;
mod grib2_section3;
mod grib2_section4;
mod grib2_section5;
mod grib2_section6;
mod grib2_section7;
mod grib2_section8;

#[cfg(test)]
mod tests;


const CLCT_TEST_FILE: &str = "./src/tests/data/icon-d2_germany_regular-lat-lon_single-level_2022041700_000_2d_clct_mod.grib2";


fn main() {
    println!("Hello, world!");
    let _layer = CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();
}
