use meteo_grib2_renderer::chart::wind_chart_renderer::WindChartRenderer;
use meteo_grib2_renderer::grib2::document::grib2_document_reader::Grib2DocumentReader;
use meteo_grib2_renderer::meteo_dwd::dwd_wind_layer::DwdWindLayer;

pub const WIND_U_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_u_10m.grib2";
pub const WIND_V_TEST_FILE: &str = "./tests/data/icon-d2_germany_regular-lat-lon_single-level_2022042600_000_2d_v_10m.grib2";


pub fn read_test_wind_layer() -> DwdWindLayer {
    let doc_u = Grib2DocumentReader::read_file(WIND_U_TEST_FILE).unwrap();
    let doc_v = Grib2DocumentReader::read_file(WIND_V_TEST_FILE).unwrap();
    let wind_layer = DwdWindLayer::from_grib2(doc_u, doc_v).unwrap();

    return wind_layer;
}

#[test]
fn it_creates_an_image_from_a_wind_layer_with_the_correct_dimensions() {
    let layer = read_test_wind_layer();

    let result = WindChartRenderer::render(&layer);
    assert!(!result.is_err());

    let result = result.unwrap();
    assert_eq!(1215, result.width());
    assert_eq!(746, result.height());
}
