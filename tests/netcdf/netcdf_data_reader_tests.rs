use std::fs::File;
use std::io::BufReader;

use meteo_grib2_renderer::netcdf::data::netcdf_data_reader::NetCdfDataReader;
use meteo_grib2_renderer::netcdf::document::netcdf_document::NetCdfDocument;
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";


fn get_doc_and_reader() -> (BufReader<File>, NetCdfDocument) {
    let file = File::open(NETCDF_ICON_GRID_TEST_FILE).unwrap();
    let mut reader = BufReader::new(file);
    let doc = NetCdfDocumentReader::open_reader(&mut reader).unwrap();

    return (reader, doc);
}


#[test]
fn it_reads_clon_clat_data_from_an_icon_grid_file() {
    let (mut reader, doc) = get_doc_and_reader();

    let result = NetCdfDataReader::read_data_by_var(&mut reader, &doc, "clon");
    assert!(result.is_ok());

    let data = result.unwrap();
    assert_eq!(5120, data.len());
}


#[test]
fn it_reads_clon_clat_data_from_an_icon_grid_file_error_when_variable_not_found() {
    let (mut reader, doc) = get_doc_and_reader();

    let result = NetCdfDataReader::read_data_by_var(&mut reader, &doc, "meep");
    assert!(result.is_err());
}
