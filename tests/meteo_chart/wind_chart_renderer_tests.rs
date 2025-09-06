use meteo_grib2_renderer::grib2::converter::file_to_grid_converter::FileToGridConverter;
use meteo_grib2_renderer::grib2::converter::regular_grid_converter::RegularGridConverter;
use meteo_grib2_renderer::meteo_chart::wind_chart_renderer::WindChartRenderer;
use meteo_grib2_renderer::meteo_layer::meteo_wind_layer::MeteoWindLayer;

pub const WIND_U_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
pub const WIND_V_TEST_FILE: &str = "./tests/resources/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";


pub fn read_test_wind_layer() -> MeteoWindLayer {
    let doc_u = FileToGridConverter::read_single_doc_from_file_or_url(WIND_U_TEST_FILE).unwrap();
    let doc_v = FileToGridConverter::read_single_doc_from_file_or_url(WIND_V_TEST_FILE).unwrap();
    let grid_u = RegularGridConverter::create(&doc_u, -1.0).unwrap();
    let grid_v = RegularGridConverter::create(&doc_v, -1.0).unwrap();
    let layer = MeteoWindLayer::new(grid_u, grid_v, None).unwrap();

    return layer;
}


#[test]
fn it_creates_an_image_from_a_wind_layer_with_the_correct_dimensions() {
    let layer = read_test_wind_layer();

    let result = WindChartRenderer::render_full_chart(&layer);
    assert!(!result.is_err());

    let result = result.unwrap();
    assert_eq!(1215, result.width());
    assert_eq!(746, result.height());
}
