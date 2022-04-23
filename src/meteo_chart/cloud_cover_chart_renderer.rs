use crate::dwd::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use crate::geo::lat_lon::LatLon;
use crate::geo::map_tile_coord::MapTileCoord;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;

pub struct CloudCoverChartRenderer;

impl CloudCoverChartRenderer {
    pub fn create_single_chart(layer: &CloudCoverLayer) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(layer.lon_grid_points(), layer.lat_grid_points())?;

        for i in 0..layer.lat_grid_points() {
            for j in 0..layer.lon_grid_points() {
                let idx = i * layer.lon_grid_points() + j;
                let value = layer.get_value_by_index(idx as usize);

                if value != CloudCoverLayer::MISSING_VALUE {
                    let color_value = (value  * 255.0).floor() as u8;
                    let color = [color_value, color_value, color_value, 255]; // TODO

                    drawable.draw_point(j, layer.lat_grid_points() - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }


    pub fn create_single_tile(layer: &CloudCoverLayer, map_tile_coords: &MapTileCoord) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(MapTileCoord::TILE_SIZE_PX, MapTileCoord::TILE_SIZE_PX)?;

        let start_pos = map_tile_coords.to_position();
        let end_pos = MapTileCoord::new(map_tile_coords.x + 1, map_tile_coords.y + 1, map_tile_coords.zoom).to_position();
        let tile_size_px = MapTileCoord::TILE_SIZE_PX as f32;
        let lon_inc = (end_pos.lon - start_pos.lon) / tile_size_px;
        let lat_inc = (end_pos.lat - start_pos.lat) / tile_size_px;

        for i in 0..MapTileCoord::TILE_SIZE_PX {
            let lat = i as f32 * lat_inc;
            for j in 0..MapTileCoord::TILE_SIZE_PX {
                let lon = j as f32 * lon_inc;
                let value = layer.get_value_by_lat_lon(&LatLon::new(lat, lon));

                if value != CloudCoverLayer::MISSING_VALUE {
                    let color_value = (value  * 255.0).floor() as u8;
                    let color = [color_value, color_value, color_value, 255]; // TODO

                    drawable.draw_point(j, MapTileCoord::TILE_SIZE_PX - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }
}
