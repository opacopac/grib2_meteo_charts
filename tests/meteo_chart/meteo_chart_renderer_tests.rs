use meteo_grib2_renderer::dwd::dwd_cloud_cover_layer::DwdCloudCoverLayer;
use meteo_grib2_renderer::geo::map_tile_coord::MapTileCoord;
use meteo_grib2_renderer::meteo_chart::map_tile_renderer::MapTileRenderer;
use meteo_grib2_renderer::meteo_chart::single_chart_renderer::SingleChartRenderer;

use crate::read_test_cloud_cover_layer;

#[test]
fn it_creates_an_image_from_a_cloud_cover_layer_with_the_correct_dimensions() {
    let layer = read_test_cloud_cover_layer();

    let result1 = SingleChartRenderer::create(
        &layer.value_grid,
        DwdCloudCoverLayer::color_by_value
    );
    assert!(!result1.is_err());

    let result2 = result1.unwrap();
    assert_eq!(1215, result2.width());
    assert_eq!(746, result2.height());
}



#[test]
fn it_creates_a_single_map_tile_from_a_cloud_cover_layer() {
    let layer = read_test_cloud_cover_layer();
    let map_tile_coords = MapTileCoord::new(0, 0, 0);

    let result1 = MapTileRenderer::create_single_tile(
        &layer.value_grid,
        &map_tile_coords,
        DwdCloudCoverLayer::color_by_value
    );
    assert!(!result1.is_err());

    let result2 = result1.unwrap();
    assert_eq!(MapTileCoord::TILE_SIZE_PX, result2.width());
    assert_eq!(MapTileCoord::TILE_SIZE_PX, result2.height());
}
