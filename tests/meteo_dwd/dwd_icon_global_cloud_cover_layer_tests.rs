use meteo_grib2_renderer::common::tstamp::TStamp;
use meteo_grib2_renderer::dwd_layer::dwd_cloud_layer::DwdCloudLayer;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::geo::lat_lon_extent::LatLonExtent;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::netcdf::converter::netcdf_to_grid_converter::{
    NetCdftoGridConverter, CLAT_VAR_NAME, CLON_VAR_NAME,
};
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const CLCT_TEST_FILE: &str =
    "./tests/resources/icon_global_icosahedral_single-level_2022051300_000_CLCT_MOD.grib2";
pub const NETCDF_FILE: &str = "./tests/resources/icon_grid_0026_R03B07_G.nc";

#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    TStamp::print_us("Grib2DocumentReader::read_single_doc_from_file...");
    let grib2_doc = Grib2DocumentReader::read_single_doc_from_file(CLCT_TEST_FILE).unwrap();
    let dimensions = (4096, 4096);
    TStamp::print_us("NetCdfDocumentReader::read_file...");
    let netcdf_doc =
        NetCdfDocumentReader::read_file(NETCDF_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME])
            .unwrap();
    TStamp::print_us("NetCdftoGridConverter::get_lat_lon_values_from_netcdf2...");
    let coordinates = NetCdftoGridConverter::get_lat_lon_values_from_netcdf2(&netcdf_doc).unwrap();
    TStamp::print_us("UnstructuredGridConverter::create...");
    let lat_lon_extent = LatLonExtent::new(
        LatLon::new(-85.0, -180.0),
        LatLon::new(85.0, 179.999)
    );
    let grid = UnstructuredGridConverter::create(
        &grib2_doc,
        -1.0,
        coordinates,
        dimensions,
        lat_lon_extent,
        0.117 // TODO
    ).unwrap();
    TStamp::print_us("grid.create_regular_grid...");
    let regular_grid = grid.create_regular_grid();
    TStamp::print_us("done.");
    let _layer = DwdCloudLayer::new(regular_grid);

    assert!(true);
}
