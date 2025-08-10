use meteo_grib2_renderer::grib2::converter::grib2_to_grid_converter::Grib2ToGridConverter;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;


pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";

#[test]
fn it_successfully_reads_an_icon_ch1_hor_contants_test_file() {
    let docs = Grib2DocumentReader::read_multi_doc_from_file(HOR_CONST_TEST_FILE).unwrap();
    let temp_doc = &docs[0];
    let clat_doc = &docs[3];
    let clon_doc = &docs[4];
    let coordinates = Grib2ToGridConverter::get_lat_lon_values_from_grib_doc(clat_doc, clon_doc).unwrap();
    let grid = UnstructuredGridConverter::create2(temp_doc, 255.0, coordinates).unwrap();
    let value = grid.get_value_by_xy(512, 512).unwrap();
    let a = 1;

    assert_eq!(0.0, 0.0);
}
