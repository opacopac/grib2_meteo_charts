use crate::read_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_document();

    let result = layer.section4.length;

    assert_eq!(34, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_document();

    let result = layer.section4.section_number;

    assert_eq!(4, result);
}
