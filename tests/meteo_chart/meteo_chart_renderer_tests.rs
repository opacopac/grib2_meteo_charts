use meteo_grib2_renderer::meteo_chart::cloud_cover_chart_renderer::CloudCoverChartRenderer;

use crate::read_test_cloud_cover_layer;

#[test]
fn it_creates_an_image_from_a_cloud_cover_layer_with_the_correct_dimensions() {
    let layer = read_test_cloud_cover_layer();

    let result1 = CloudCoverChartRenderer::create(layer);
    assert!(!result1.is_err());

    let result2 = result1.unwrap();
    assert_eq!(1215, result2.width());
    assert_eq!(746, result2.height());
}
