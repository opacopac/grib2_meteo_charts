use meteo_grib2_renderer::grib2_section0::discipline::Discipline;

use crate::test_common::read_test_layer;

mod test_common;

#[test]
fn it_reads_the_correct_discipline() {
    let layer = read_test_layer();

    let result = layer.section0.discipline;

    assert_eq!(Discipline::Meteorological, result);
}


#[test]
fn it_reads_the_correct_edition() {
    let layer = read_test_layer();

    let result = layer.section0.edition;

    assert_eq!(2, result);
}


#[test]
fn it_reads_the_correct_length() {
    let layer = read_test_layer();

    let result = layer.section0.length;

    assert_eq!(1623229, result);
}
