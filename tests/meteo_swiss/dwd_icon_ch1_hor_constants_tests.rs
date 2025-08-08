use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::regular_grid_converter::RegularGridConverter;

pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";

#[test]
fn it_successfully_reads_an_icon_ch1_hor_contants_test_file() {
    let doc = Grib2DocumentReader::read_file(HOR_CONST_TEST_FILE).unwrap();
    let grid = RegularGridConverter::create(&doc, -1.0).unwrap();

    assert_eq!(0.0, 1.0);
}
