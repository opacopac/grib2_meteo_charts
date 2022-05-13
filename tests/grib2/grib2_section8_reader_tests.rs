use crate::read_icon_d2_test_document;

#[test]
fn it_reads_the_correct_end_magic() {
    let layer = read_icon_d2_test_document();

    let result = layer.section8.end_magic;

    assert_eq!("7777", result);
}
