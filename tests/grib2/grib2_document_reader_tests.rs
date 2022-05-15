use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;

use crate::{DATA_DIR, read_icon_d2_test_document_result};

#[test]
fn it_reads_an_existing_grib2_file() {
    let result = read_icon_d2_test_document_result();

    assert!(result.is_ok());
}


#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let grib2_file =  DATA_DIR.to_string() + "notfound.grib2";

    let result = Grib2DocumentReader::read_file(&grib2_file);

    assert!(result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_grib2_format() {
    let grib2_file = DATA_DIR.to_string() + "not_a_grib2_file.grib2";

    let result = Grib2DocumentReader::read_file(&grib2_file);

    assert!(result.is_err());
}
