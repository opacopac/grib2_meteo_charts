use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;

pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";

#[test]
fn it_successfully_reads_an_icon_ch1_hor_contants_test_file() {
    let docs = Grib2DocumentReader::read_multi_doc_from_file(HOR_CONST_TEST_FILE).unwrap();
    let lon_values = docs[3].calculate_data_points(255.0, |x| x as f32).unwrap();
    let lat_values = docs[4].calculate_data_points(255.0, |x| x as f32).unwrap();
    let lon_size = lon_values.len();
    let lat_size = lat_values.len();
    let grid = UnstructuredGridConverter::create2(&docs[0], -1.0, &docs[3], &docs[4]).unwrap();
    let a = 1;

    assert_eq!(0.0, 0.0);
}
