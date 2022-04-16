use crate::grib2_cloud_cover::grib2_cloud_cover_reader::Grib2CloudCoverReader;
use crate::grib2_section0::grib2_discipline::Grib2Discipline;

const CLCT_TEST_FILE: &str = "icon_global_icosahedral_single-level_2022041500_000_CLCT.grib2";


#[test]
fn it_reads_the_correct_discipline() {
    let layer = Grib2CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();

    let result = layer.section0.discipline;

    assert_eq!(Grib2Discipline::Meteorological, result);
}


#[test]
fn it_reads_the_correct_edition() {
    let layer = Grib2CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();

    let result = layer.section0.edition;

    assert_eq!(2, result);
}
