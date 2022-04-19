use crate::test_common::read_test_layer;

mod test_common;

#[test]
fn it_reads_the_correct_end_magic() {
    let layer = read_test_layer();

    let result = layer.section8.end_magic;

    assert_eq!("7777", result);
}
