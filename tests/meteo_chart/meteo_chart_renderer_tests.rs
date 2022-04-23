use meteo_grib2_renderer::geo::map_tile_coord::MapTileCoord;
use meteo_grib2_renderer::meteo_chart::cloud_cover_chart_renderer::CloudCoverChartRenderer;

use crate::read_test_cloud_cover_layer;

#[test]
fn it_creates_an_image_from_a_cloud_cover_layer_with_the_correct_dimensions() {
    let layer = read_test_cloud_cover_layer();

    let result1 = CloudCoverChartRenderer::create_single_chart(&layer);
    assert!(!result1.is_err());

    let result2 = result1.unwrap();
    assert_eq!(1215, result2.width());
    assert_eq!(746, result2.height());
}



#[test]
fn it_creates_a_single_map_tile_from_a_cloud_cover_layer() {
    let layer = read_test_cloud_cover_layer();
    let map_tile_coords = MapTileCoord::new(0, 0, 0);

    let result1 = CloudCoverChartRenderer::create_single_tile(&layer, &map_tile_coords);
    assert!(!result1.is_err());

    let result2 = result1.unwrap();
    assert_eq!(MapTileCoord::TILE_SIZE_PX, result2.width());
    assert_eq!(MapTileCoord::TILE_SIZE_PX, result2.height());
}
