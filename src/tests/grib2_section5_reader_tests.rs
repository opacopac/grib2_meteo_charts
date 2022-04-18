use crate::tests::test_common::read_test_layer;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section5.length;

    assert_eq!(21, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section5.section_number;

    assert_eq!(5, result);
}


#[test]
fn it_reads_the_correct_number_of_data_points() {
    let layer = read_test_layer();

    let result = layer.section5.data_points;

    assert_eq!(754862, result);
}
