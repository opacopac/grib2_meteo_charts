use crate::read_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_document();

    let result = layer.section2.length;

    assert_eq!(27, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_document();

    let result = layer.section2.section_number;

    assert_eq!(2, result);
}
