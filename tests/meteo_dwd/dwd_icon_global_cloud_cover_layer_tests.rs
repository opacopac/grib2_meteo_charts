use meteo_grib2_renderer::chart::temp_chart_renderer::TempChartRenderer;
use meteo_grib2_renderer::common::tstamp::TStamp;
use meteo_grib2_renderer::dwd_layer::dwd_temp_layer::DwdTempLayer;
use meteo_grib2_renderer::geo::lat_lon::LatLon;
use meteo_grib2_renderer::geo::lat_lon_extent::LatLonExtent;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::grid::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::netcdf::converter::netcdf_to_grid_converter::{
    NetCdftoGridConverter, CLAT_VAR_NAME, CLON_VAR_NAME,
};
use meteo_grib2_renderer::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub const T_2M_GRIB_FILE: &str =
    "./tests/resources/icon_global_icosahedral_single-level_2025081318_000_T_2M.grib2";
pub const NETCDF_FILE: &str = "./tests/resources/icon_grid_0026_R03B07_G.nc";
pub const CHART_OUTPUT_FILE: &str = "./icon-global-t_2m-chart.png";

#[test]
fn it_successfully_reads_an_icon_global_clct_test_file() {
    TStamp::print_us("Grib2DocumentReader::read_single_doc_from_file...");
    let grib2_doc = Grib2DocumentReader::read_single_doc_from_file(T_2M_GRIB_FILE).unwrap();
    let dimensions = (4096, 4096);
    TStamp::print_us("NetCdfDocumentReader::read_file...");
    let netcdf_doc =
        NetCdfDocumentReader::read_file(NETCDF_FILE, vec![CLAT_VAR_NAME, CLON_VAR_NAME]).unwrap();
    TStamp::print_us("NetCdftoGridConverter::get_lat_lon_values_from_netcdf2...");
    let coordinates = NetCdftoGridConverter::get_lat_lon_values_from_netcdf2(&netcdf_doc).unwrap();
    TStamp::print_us("UnstructuredGridConverter::create...");
    let grid = UnstructuredGridConverter::create(
        &grib2_doc,
        |x| x - 273.15, // convert Kelvin to Celsius,
        255.0,
        coordinates,
        dimensions,
        LatLonExtent::MERCATOR_EXTENT,
        0.117, // TODO
    )
    .unwrap();
    TStamp::print_us("grid.create_regular_grid...");
    let regular_grid = grid.create_regular_grid();

    TStamp::print_us("DwdTempLayer::new...");
    let dwd_temp_layer = DwdTempLayer::new(regular_grid).unwrap();

    TStamp::print_us("TempChartRenderer::render_full_chart...");
    let drawable = TempChartRenderer::render_full_chart(&dwd_temp_layer).unwrap();

    TStamp::print_us("Drawable::safe_image...");
    drawable.safe_image(CHART_OUTPUT_FILE).unwrap();

    TStamp::print_us("done.");

    assert!(true);
}
