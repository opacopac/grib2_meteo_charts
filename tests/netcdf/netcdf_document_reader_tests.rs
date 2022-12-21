use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/resources/icon_grid_0009_R02B03_R.nc";
pub const NOT_FOUND_TEST_FILE: &str = "./tests/resources/notfound.nc";
pub const NON_NETCDF_TEST_FILE: &str = "./tests/resources/not_a_grib2_file.grib2";


#[test]
fn it_opens_an_existing_netcdf_file() {
    let result = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec![]);
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(14, doc.header.dim_list.len());
    assert_eq!(21, doc.header.att_list.len());
    assert_eq!(69, doc.header.var_list.len());
    assert_eq!(0, doc.data_map.len());
}


#[test]
fn it_returns_an_error_if_the_file_doesnt_exist() {
    let result = NetCdfDocumentReader::read_file(NOT_FOUND_TEST_FILE, vec![]);

    assert!(result.is_err());
}


#[test]
fn it_returns_an_error_if_the_file_isnt_in_netcdf_format() {
    let result = NetCdfDocumentReader::read_file(NON_NETCDF_TEST_FILE, vec![]);

    assert!(result.is_err());
}


#[test]
fn it_reads_clon_data_from_an_icon_grid_file() {
    let result = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec!["clon"]);
    assert!(result.is_ok());

    let doc = result.unwrap();
    let values = doc.data_map["clon"].get_doubles();
    assert_eq!(5120, values.len())
}


#[test]
fn it_reads_clon_clat_data_from_an_icon_grid_file_error_when_variable_not_found() {
    let result = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE, vec!["meep"]);
    assert!(result.is_err());
}
