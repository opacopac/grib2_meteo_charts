use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::netcdf::converter::netcdf_to_grid_converter::{CLAT_VAR_NAME, CLON_VAR_NAME};
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const CLCT_TEST_FILE: &str = "./tests/resources/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";


#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    let grib2_doc = Grib2DocumentReader::read_single_doc_from_file(CLCT_TEST_FILE).unwrap();
    let netcdf_doc = NetCdfDocumentReader::read_file(CLCT_TEST_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap(); // TODO
    let grid = UnstructuredGridConverter::create(&grib2_doc, -1.0, &netcdf_doc).unwrap();
    let _layer = DwdCloudLayer::new(grid);

    // TODO: panics because number of points in grid don't match

    assert!(true);
}
