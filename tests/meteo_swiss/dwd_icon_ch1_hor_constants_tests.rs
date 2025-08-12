use meteo_grib2_renderer::common::tstamp::TStamp;
use meteo_grib2_renderer::grib2::converter::grib2_to_grid_converter::Grib2ToGridConverter;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;


pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";
pub const T2M_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202508100900-0-t_2m-ctrl.grib2";

#[test]
fn it_successfully_reads_an_icon_ch1_hor_contants_test_file() {
    TStamp::print("Grib2DocumentReader::read_multi_doc_from_file...");
    let hor_docs = Grib2DocumentReader::read_multi_doc_from_file(HOR_CONST_TEST_FILE).unwrap();

    let clat_doc = &hor_docs[3];
    let clon_doc = &hor_docs[4];

    TStamp::print("Grib2ToGridConverter::get_lat_lon_values_from_grib_doc...");
    let coordinates = Grib2ToGridConverter::get_lat_lon_values_from_grib_doc(clat_doc, clon_doc).unwrap();

    let dimensions = (1024, 1024);

    TStamp::print("Grib2DocumentReader::read_single_doc_from_file...");
    let t2m_doc = Grib2DocumentReader::read_single_doc_from_file(T2M_TEST_FILE).unwrap();

    TStamp::print("UnstructuredGridConverter::create...");
    let grid = UnstructuredGridConverter::create(
        &t2m_doc,
        255.0, // TODO
        coordinates,
        dimensions,
        0.01 // TODO
    ).unwrap();

    TStamp::print("grid.create_regular_grid...");
    let regrid = grid.create_regular_grid();
    let a = 1;

    TStamp::print("done.");

    assert_eq!(0.0, 0.0);
}
