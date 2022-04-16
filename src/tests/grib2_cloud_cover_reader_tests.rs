use crate::grib2_cloud_cover::grib2_cloud_cover_reader::Grib2CloudCoverReader;

const CLCT_TEST_FILE: &str = "icon_global_icosahedral_single-level_2022041500_000_CLCT.grib2";

#[test]
fn it_reads_an_existing_grib2_file() {
    let result = Grib2CloudCoverReader::read_file(CLCT_TEST_FILE);

    assert_eq!(false, result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let grib2_file = "notfound.grib2";

    let result = Grib2CloudCoverReader::read_file(grib2_file);

    assert_eq!(true, result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_grib2_format() {
    let grib2_file = "not_a_grib2_file.grib2";

    let result = Grib2CloudCoverReader::read_file(grib2_file);

    assert_eq!(true, result.is_err());
}
