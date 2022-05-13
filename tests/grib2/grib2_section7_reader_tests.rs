use crate::read_icon_d2_test_document;

#[test]
fn it_reads_the_correct_section_length() {
    let layer = read_icon_d2_test_document();

    let result = layer.section7.length;

    assert_eq!(1509729, result);
}


#[test]
fn it_reads_the_correct_section_number() {
    let layer = read_icon_d2_test_document();

    let result = layer.section7.section_number;

    assert_eq!(7, result);
}


#[test]
fn it_reads_all_data_points() {
    let layer = read_icon_d2_test_document();

    let result = layer.section7.data_points;

    assert_eq!(754862, result.len());
}
