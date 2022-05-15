use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";

#[test]
fn it_reads_an_existing_grib2_file() {
    let result = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE);

    assert!(result.is_ok());
}


/*#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let grib2_file =  DATA_DIR.to_string() + "notfound.grib2";

    let result = Grib2DocumentReader::read_file(&grib2_file);

    assert_eq!(true, result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_grib2_format() {
    let grib2_file = DATA_DIR.to_string() + "not_a_grib2_file.grib2";

    let result = Grib2DocumentReader::read_file(&grib2_file);

    assert_eq!(true, result.is_err());
}
*/
