use std::fs::File;
use std::io::BufReader;

use meteo_grib2_renderer::netcdf::data::netcdf_data_reader::NetCdfDataReader;
use meteo_grib2_renderer::netcdf::document::netcdf_document::NetCdfDocument;
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";


fn get_doc_and_reader() -> (NetCdfDocument, BufReader<File>) {
    let (doc, reader) = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE).unwrap();

    return (doc, reader);
}


#[test]
fn it_reads_clon_clat_data_from_an_icon_grid_file() {
    let (doc, mut reader) = get_doc_and_reader();

    let result = NetCdfDataReader::read_data_by_var(&mut reader, &doc, "clon");
    assert!(result.is_ok());

    let data = result.unwrap();
    let values = data.get_doubles();
    assert_eq!(5120, values.len())
}


#[test]
fn it_reads_clon_clat_data_from_an_icon_grid_file_error_when_variable_not_found() {
    let (doc, mut reader) = get_doc_and_reader();

    let result = NetCdfDataReader::read_data_by_var(&mut reader, &doc, "meep");
    assert!(result.is_err());
}
