use meteo_grib2_renderer::chart::temp_chart_renderer::TempChartRenderer;
use meteo_grib2_renderer::geo::lat_lon_extent::LatLonExtent;
use meteo_grib2_renderer::grib2::converter::grib2_to_grid_converter::Grib2ToGridConverter;
use meteo_grib2_renderer::grib2::converter::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::logging::tstamp::TStamp;
use meteo_grib2_renderer::meteo_layer::meteo_temp_layer::MeteoTempLayer;


pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";
pub const T2M_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202508100900-0-t_2m-ctrl.grib2";
pub const CHART_OUTPUT_FILE: &str = "./icon-ch1-t_2m-chart.png";


#[test]
fn it_successfully_reads_an_icon_ch1_hor_contants_test_file() {
    TStamp::print_us("Grib2DocumentReader::read_multi_doc_from_file...");
    let hor_docs = Grib2DocumentReader::read_multi_doc_from_file(HOR_CONST_TEST_FILE).unwrap();

    let clat_doc = &hor_docs[4];
    let clon_doc = &hor_docs[3];

    TStamp::print_us("Grib2ToGridConverter::get_lat_lon_values_from_grib_doc...");
    let coordinates =
        Grib2ToGridConverter::get_lat_lon_values_from_grib_doc(clat_doc, clon_doc).unwrap();

    let dimensions = (1024, 1024);

    TStamp::print_us("Grib2DocumentReader::read_single_doc_from_file...");
    let t2m_doc = Grib2DocumentReader::read_single_doc_from_file(T2M_TEST_FILE).unwrap();


    TStamp::print_us("UnstructuredGridConverter::create...");
    let lat_lon_extent = LatLonExtent::calc_min_bounding_extent(&coordinates);
    let grid = UnstructuredGridConverter::create(
        &t2m_doc,
        |x| x - 273.15, // convert Kelvin to Celsius
        255.0,          // TODO
        coordinates,
        dimensions,
        lat_lon_extent,
        0.01, // TODO
    ).unwrap();

    TStamp::print_us("grid.create_regular_grid...");
    let regular_grid = grid.create_regular_grid();

    TStamp::print_us("DwdTempLayer::new...");
    let dwd_temp_layer = MeteoTempLayer::new(regular_grid).unwrap();

    TStamp::print_us("TempChartRenderer::render_full_chart...");
    let drawable = TempChartRenderer::render_full_chart(&dwd_temp_layer).unwrap();

    TStamp::print_us("Drawable::safe_image...");
    drawable.safe_image(CHART_OUTPUT_FILE).unwrap();

    TStamp::print_us("done.");

    assert_eq!(0.0, 0.0);
}
