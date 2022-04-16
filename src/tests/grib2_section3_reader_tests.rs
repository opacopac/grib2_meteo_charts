use crate::grib2_cloud_cover::grib2_cloud_cover_layer::Grib2CloudCoverLayer;
use crate::grib2_cloud_cover::grib2_cloud_cover_reader::Grib2CloudCoverReader;
use crate::grib2_section3::grib2_grid_definition_source::Grib2GridDefinitionSource;

const CLCT_TEST_FILE: &str = "icon_global_icosahedral_single-level_2022041500_000_CLCT.grib2";


fn read_test_layer() -> Grib2CloudCoverLayer {
    return Grib2CloudCoverReader::read_file(CLCT_TEST_FILE).unwrap();
}


#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section3.length;

    assert_eq!(35, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section3.section_number;

    assert_eq!(3, result);
}


#[test]
fn it_reads_the_correct_grid_definition_source() {
    let layer = read_test_layer();

    let result = layer.section3.grid_definition_source;

    assert_eq!(Grib2GridDefinitionSource::GridDefinitionTemplate, result);
}
