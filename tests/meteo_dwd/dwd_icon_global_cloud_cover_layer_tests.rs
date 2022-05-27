use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_cloud_layer2::DwdCloudLayer2;
use meteo_grib2_renderer::meteo_dwd::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::netcdf::data::netcdf_data_reader::NetCdfDataReader;
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const CLCT_TEST_FILE: &str = "./tests/data/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";
pub const NETCDF_ICON_GRID_TEST_FILE: &str = "./tests/data/icon_grid_0009_R02B03_R.nc";


#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    let grib2_doc = Grib2DocumentReader::read_file(CLCT_TEST_FILE).unwrap();
    let (netcdf_doc, mut reader) = NetCdfDocumentReader::read_file(NETCDF_ICON_GRID_TEST_FILE).unwrap();
    let clat_data = NetCdfDataReader::read_data_by_var(&mut reader, &netcdf_doc, "clat").unwrap().get_doubles();
    let clon_data = NetCdfDataReader::read_data_by_var(&mut reader, &netcdf_doc, "clon").unwrap().get_doubles();
    let grid = UnstructuredGridConverter::create(&grib2_doc, -1.0, clat_data, clon_data).unwrap();
    let _layer = DwdCloudLayer2::new(grid);

    // TODO: panics because number of points in grid don't match

    assert!(true);
}
