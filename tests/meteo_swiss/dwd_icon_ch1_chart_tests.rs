use meteo_grib2_renderer::geo::common::lat_lon::LatLon;
use meteo_grib2_renderer::geo::common::lat_lon_extent::LatLonExtent;
use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::converter::grib2_to_grid_converter::Grib2ToGridConverter;
use meteo_grib2_renderer::grib2::converter::unstructured_grid_converter::UnstructuredGridConverter;
use meteo_grib2_renderer::meteo_chart::forecast_renderer::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use meteo_grib2_renderer::meteo_chart::forecast_renderer::temp_2m_chart_renderer::Temp2mChartRenderer;
use meteo_grib2_renderer::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use meteo_grib2_renderer::meteo_chart::meteo_layer::meteo_temp_2m_layer::MeteoTemp2mLayer;
use meteo_grib2_renderer::physics::temperature::Temperature;
use meteo_grib2_renderer::system::tstamp::TStamp;


pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";
pub const T2M_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202508100900-0-t_2m-ctrl.grib2";
pub const CLCT_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202509080600-11-clct-ctrl.grib2";
pub const TOT_PREC1_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202509080600-10-tot_prec-ctrl.grib2";
pub const TOT_PREC2_TEST_FILE: &str = "./tests/resources/icon-ch1-eps-202509080600-11-tot_prec-ctrl.grib2";
pub const T2M_CHART_OUTPUT_FILE: &str = "./icon-ch1-t_2m-meteo_chart.png";
pub const CLCT_PREC_CHART_OUTPUT_FILE: &str = "./icon-ch1-clct-precip-meteo_chart.png";
pub const CHART_IMG_DIM: (usize, usize) = (1851, 847);
pub const MAX_COORD_DIST_DEG: f32 = 0.01;


fn get_coordinates() -> Vec<LatLon> {
    TStamp::print_us("FileToGridConverter::read_multi_doc_from_file_or_url...");
    let hor_docs = FileToGridConverter::read_multi_doc_from_file_or_url(HOR_CONST_TEST_FILE).unwrap();
    let clat_doc = &hor_docs[4];
    let clon_doc = &hor_docs[3];

    TStamp::print_us("Grib2ToGridConverter::get_lat_lon_values_from_grib_doc...");
    let coordinates = Grib2ToGridConverter::get_lat_lon_values_from_grib_doc(clat_doc, clon_doc).unwrap();

    coordinates
}


#[test]
fn it_successfully_renders_an_icon_ch1_temp_2m_chart() {
    let coordinates = get_coordinates();

    TStamp::print_us("FileToGridConverter::read_single_doc_from_file...");
    let t2m_doc = FileToGridConverter::read_single_doc_from_file_or_url(T2M_TEST_FILE).unwrap();

    TStamp::print_us("UnstructuredGridConverter::create...");
    // let lat_lon_extent = LatLonExtent::calc_min_bounding_extent(&coordinates);
    let lat_lon_extent = LatLonExtent::new(
        LatLon::new(42.03, -0.81),
        LatLon::new(50.50, 17.70),
    );
    let grid = UnstructuredGridConverter::create(
        &t2m_doc,
        |x| Temperature::from_kelvin_to_celsius(x),
        255.0,          // TODO
        coordinates,
        CHART_IMG_DIM,
        lat_lon_extent,
        MAX_COORD_DIST_DEG,
    ).unwrap();

    TStamp::print_us("grid.create_regular_grid...");
    let regular_grid = grid.create_regular_grid();

    TStamp::print_us("DwdTempLayer::new...");
    let dwd_temp_layer = MeteoTemp2mLayer::new(regular_grid);

    TStamp::print_us("TempChartRenderer::render_full_chart...");
    let drawable = Temp2mChartRenderer::render_full_chart(&dwd_temp_layer).unwrap();

    TStamp::print_us("Drawable::safe_image...");
    drawable.safe_image(T2M_CHART_OUTPUT_FILE).unwrap();

    TStamp::print_us("done.");

    assert_eq!(0.0, 0.0);
}

#[test]
fn it_successfully_renders_an_icon_ch1_clct_precip_chart() {
    let coordinates = get_coordinates();

    TStamp::print_us("FileToGridConverter::read_single_doc_from_file...");
    let clct_doc = FileToGridConverter::read_single_doc_from_file_or_url(CLCT_TEST_FILE).unwrap();
    let tot_prec_doc1 = FileToGridConverter::read_single_doc_from_file_or_url(TOT_PREC1_TEST_FILE).unwrap();
    let tot_prec_doc2 = FileToGridConverter::read_single_doc_from_file_or_url(TOT_PREC2_TEST_FILE).unwrap();

    TStamp::print_us("UnstructuredGridConverter::create...");
    let lat_lon_extent = LatLonExtent::calc_min_bounding_extent(&coordinates);
    let clct_grid = UnstructuredGridConverter::create(
        &clct_doc,
        |x| x, // no conversion
        -1.0, // TODO
        coordinates.clone(),
        CHART_IMG_DIM,
        lat_lon_extent.clone(),
        MAX_COORD_DIST_DEG,
    ).unwrap();
    let tot_prec_grid1 = UnstructuredGridConverter::create(
        &tot_prec_doc1,
        |x| x, // no conversion
        -1.0, // TODO
        coordinates.clone(),
        CHART_IMG_DIM,
        lat_lon_extent.clone(),
        MAX_COORD_DIST_DEG,
    ).unwrap();
    let tot_prec_grid2 = UnstructuredGridConverter::create(
        &tot_prec_doc2,
        |x| x, // no conversion
        -1.0, // TODO
        coordinates.clone(),
        CHART_IMG_DIM,
        lat_lon_extent.clone(),
        MAX_COORD_DIST_DEG,
    ).unwrap();


    TStamp::print_us("clct_grid.create_regular_grid...");
    let clct_regular_grid = clct_grid.create_regular_grid();
    TStamp::print_us("tot_prec1_grid.create_regular_grid...");
    let tot_prec1_regular_grid = tot_prec_grid1.create_regular_grid();
    TStamp::print_us("tot_prec2_grid.create_regular_grid...");
    let tot_prec2_regular_grid = tot_prec_grid2.create_regular_grid();

    TStamp::print_us("MeteoCloudPrecipLayer::new...");
    let meteo_cloud_precip_layer = MeteoCloudPrecipLayer::new(
        clct_regular_grid,
        tot_prec1_regular_grid,
        tot_prec2_regular_grid,
    ).unwrap();

    TStamp::print_us("CloudPrecipChartRenderer::render_full_chart...");
    let drawable = CloudPrecipChartRenderer::render_full_chart(&meteo_cloud_precip_layer).unwrap();

    TStamp::print_us("Drawable::safe_image...");
    drawable.safe_image(CLCT_PREC_CHART_OUTPUT_FILE).unwrap();

    TStamp::print_us("done.");

    assert_eq!(0.0, 0.0);
}
