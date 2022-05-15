use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";
pub const NOT_FOUND_TEST_FILE: &str = "./tests/data/notfound.nc";
pub const NON_NETCDF_TEST_FILE: &str = "./tests/data/not_a_grib2_file.grib2";


#[test]
fn it_reads_an_existing_grib2_file() {
    let result = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE);

    assert!(result.is_ok());

    let doc = result.unwrap();

    println!("{}", doc.header);
}


#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let result = NetCdfDocumentReader::read_file(NOT_FOUND_TEST_FILE);

    assert!(result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_grib2_format() {
    let result = NetCdfDocumentReader::read_file(NON_NETCDF_TEST_FILE);

    assert!(result.is_err());
}
