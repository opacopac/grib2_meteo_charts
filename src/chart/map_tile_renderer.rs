use min_max::{max, min};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::geo::map_tile_coord::MapTileCoord;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;

pub struct MapTileRenderer;


impl MapTileRenderer {
    pub fn create_all_tiles<V, C, S, T>(
        lat_long_extent: &LatLonExtent,
        zoom_range: (u32, u32),
        value_fn: V,
        color_fn: C,
        save_fn: S
    ) -> Result<(), Grib2Error> where
        V: Fn(&LatLon) -> Option<T> + Sync,
        C: Fn(T) -> [u8; 4] + Sync,
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let pos_tl = LatLon { lat: lat_long_extent.min_coord.lat, lon: lat_long_extent.max_coord.lon };
        let pos_br = LatLon { lat: lat_long_extent.max_coord.lat, lon: lat_long_extent.min_coord.lon };

        for zoom in zoom_range.0..=zoom_range.1 {
            let tile_tl = MapTileCoord::from_position(&pos_tl, zoom);
            let tile_br = MapTileCoord::from_position(&pos_br, zoom);
            let x_range = (min(tile_tl.x, tile_br.x), max(tile_tl.x, tile_br.x));
            let y_range = (min(tile_tl.y, tile_br.y), max(tile_tl.y, tile_br.y));
            println!("zoom: {}, x-range: {:?}, y-range:{:?}", zoom, x_range, y_range);

            for x in x_range.0..=x_range.1 {
                print!("x: {}, y:", x);
                (y_range.0..=y_range.1).into_par_iter().for_each(|y| {
                    print!(" {}", y);
                    // println!("rendering tile x: {}, y: {}, z: {}", x, y, zoom);
                    let map_tile_coords = &MapTileCoord::new(x, y, zoom);
                    let tile = Self::create_single_tile(&value_fn, map_tile_coords, &color_fn).unwrap(); // TODO
                    save_fn(&tile, zoom, x, y);
                });
                println!();
            }
        }

        Ok(())
    }


    pub fn create_single_tile<V, C, T>(
        value_fn: V,
        map_tile_coords: &MapTileCoord,
        color_fn: C
    ) -> Result<Drawable, Grib2Error> where
        V: Fn(&LatLon) -> Option<T> + Sync,
        C: Fn(T) -> [u8; 4] + Sync,
    {
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
                let value = value_fn(&LatLon::new(lat, lon));

                match value {
                    Some(v) => {
                        let color = color_fn(v);
                        drawable.draw_point(j, i, color);
                    }
                    _ => {}
                }
            }
        }

        return Ok(drawable);
    }
}
