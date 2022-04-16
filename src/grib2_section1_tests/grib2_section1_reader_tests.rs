use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;
use crate::grib2_cloud_cover::grib2_cloud_cover_reader::Grib2CloudCoverReader;

const CLCT_TEST_FILE: &str = "icon_global_icosahedral_single-level_2022041500_000_CLCT.grib2";


fn read_test_layer() -> Grib2CloudCoverLayer {
    return Grib2CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();
}


#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section1.length;

    assert_eq!(true, result == 21);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section1.section_number;

    assert_eq!(1, result);
}


#[test]
fn it_reads_the_correct_center_and_subcenter() {
    let layer = read_test_layer();

    let result1 = layer.section1.center;
    let result2 = layer.section1.subcenter;

    assert_eq!(78, result1); // Offenbach
    assert_eq!(255, result2); // none
}


#[test]
fn it_reads_the_correct_grib_master_and_sub_table_numbers() {
    let layer = read_test_layer();

    let result1 = layer.section1.master_table_version;
    let result2 = layer.section1.local_table_version;

    assert_eq!(19, result1);
    assert_eq!(1, result2);
}
