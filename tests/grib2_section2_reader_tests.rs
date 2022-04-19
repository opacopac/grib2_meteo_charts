use crate::test_common::read_test_layer;

mod test_common;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_layer();

    let result = layer.section2.length;

    assert_eq!(27, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_layer();

    let result = layer.section2.section_number;

    assert_eq!(2, result);
}
