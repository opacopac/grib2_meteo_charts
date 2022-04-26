use std::fs;

use min_max::{max, min};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::dwd::value_grid::ValueGrid;
use crate::geo::lat_lon::LatLon;
use crate::geo::map_tile_coord::MapTileCoord;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;

pub struct ValueGridChartRenderer;

impl ValueGridChartRenderer {
    pub fn create_single_chart(value_grid: &ValueGrid) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(value_grid.grid.lon_grid_points, value_grid.grid.lat_grid_points)?;

        for i in 0..value_grid.grid.lat_grid_points {
            for j in 0..value_grid.grid.lon_grid_points {
                let idx = i * value_grid.grid.lon_grid_points + j;
                let value = value_grid.get_value_by_index(idx as usize);

                if value != ValueGrid::MISSING_VALUE {
                    let color = Self::get_color(value);

                    drawable.draw_point(j, value_grid.grid.lat_grid_points - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }


    pub fn create_single_tile(value_grid: &ValueGrid, map_tile_coords: &MapTileCoord) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(MapTileCoord::TILE_SIZE_PX, MapTileCoord::TILE_SIZE_PX)?;

        let start_pos = map_tile_coords.to_position();
        let end_pos = MapTileCoord::new(map_tile_coords.x + 1, map_tile_coords.y + 1, map_tile_coords.zoom).to_position();
        let tile_size_px = MapTileCoord::TILE_SIZE_PX as f32;
        let lon_inc = (end_pos.lon - start_pos.lon) / tile_size_px;
        let lat_inc = (end_pos.lat - start_pos.lat) / tile_size_px;

        for i in 0..MapTileCoord::TILE_SIZE_PX {
            let lat = start_pos.lat + i as f32 * lat_inc;
            for j in 0..MapTileCoord::TILE_SIZE_PX {
                let lon = start_pos.lon + j as f32 * lon_inc;
                let value = value_grid.get_value_by_lat_lon(&LatLon::new(lat, lon));

                if value != ValueGrid::MISSING_VALUE {
                    let color = Self::get_color(value);

                    drawable.draw_point(j, i, color);
                }
            }
        }

        return Ok(drawable);
    }


    pub fn create_all_tiles(
        value_grid: &ValueGrid,
        zoom_range: (u32, u32),
        base_path: &str
    ) -> Result<(), Grib2Error> {
        let min_pos = value_grid.grid.get_min_pos();
        let max_pos = value_grid.grid.get_max_pos();
        let pos_tl = LatLon::new(min_pos.lat, max_pos.lon);
        let pos_br = LatLon::new(max_pos.lat, min_pos.lon);

        for zoom in zoom_range.0..=zoom_range.1 {
            let tile_tl = MapTileCoord::from_position(&pos_tl, zoom);
            let tile_br = MapTileCoord::from_position(&pos_br, zoom);
            let x_range = (min(tile_tl.x, tile_br.x), max(tile_tl.x, tile_br.x));
            let y_range = (min(tile_tl.y, tile_br.y), max(tile_tl.y, tile_br.y));

            for x in x_range.0..=x_range.1 {
                (y_range.0..=y_range.1).into_par_iter().for_each(|y| {
                    println!("rendering tile x: {}, y: {}, z: {}", x, y, zoom);
                    let map_tile_coords = &MapTileCoord::new(x, y, zoom);
                    let tile = Self::create_single_tile(value_grid, map_tile_coords).unwrap(); // TODO
                    Self::save_tile(&tile, zoom, x, y, base_path);
                })
            }
        }

        Ok(())
    }


    fn get_color(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [255, 255, 255, u8_value]; // TODO
    }


    fn save_tile(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        base_path: &str
    ) {
        let path = format!("{}/{}/{}", base_path, zoom, x);
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let _result = tile.safe_image(&filename);
    }
}
