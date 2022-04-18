use crate::grib2_cloud_cover::cloud_cover_reader::CloudCoverReader;

mod grib2_common;
mod grib2_cloud_cover;
mod grib2_section0;
mod grib2_section1;
mod grib2_section2;
mod grib2_section3;

#[cfg(test)]
mod tests;
mod grib2_section4;
mod grib2_section5;
mod grib2_section6;


const CLCT_TEST_FILE: &str = "icon_global_icosahedral_single-level_2022041500_000_CLCT.grib2";


fn main() {
    println!("Hello, world!");
    let _layer = CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();
}
