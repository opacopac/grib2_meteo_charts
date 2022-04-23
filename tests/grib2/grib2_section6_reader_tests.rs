use crate::read_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_test_document();

    let result = layer.section6.length;

    assert_eq!(113305, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_test_document();

    let result = layer.section6.section_number;

    assert_eq!(6, result);
}


#[test]
fn it_reads_the_correct_bitmap_indicator() {
    let layer = read_test_document();

    let result = layer.section6.bitmap_indicator;

    assert_eq!(0, result);
}


#[test]
fn it_reads_the_correct_bitmap() {
    let layer = read_test_document();

    let result = layer.section6.bitmap;
    let expected = ((1215.0 * 746.0 / 8.0) as f64).ceil() as usize;

    assert_eq!(expected, result.len());
}
