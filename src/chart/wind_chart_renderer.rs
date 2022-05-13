use std::fs;
use min_max::{max, min};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chart::wind_arrow_service::WindArrowService;
use crate::geo::lat_lon::LatLon;
use crate::geo::map_tile_coord::MapTileCoord;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::imaging::image::Image;
use crate::meteo::wind_layer::WindLayer;
use crate::meteo_dwd::dwd_wind_layer::DwdWindLayer;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct WindChartRenderer;


impl WindChartRenderer {
    const MAX_VALUE_MPS: f32 = 20.0;
    const WIND_DIR_DIST_PX: u32 = 50;


    pub fn render_full_chart(wind_layer: &DwdWindLayer) -> Result<Drawable, Grib2Error> {
        let grid_points = wind_layer.get_latlon_grid_points();
        let mut drawable = Drawable::create_empty(grid_points.1, grid_points.0)?;

        Self::draw_color_bg(wind_layer, &mut drawable);
        Self::draw_wind_arrows(wind_layer, &mut drawable)?;

        return Ok(drawable);
    }


    pub fn create_all_tiles(
        wind_layer: &DwdWindLayer,
        zoom_range: (u32, u32),
        base_path: &str
    ) -> Result<(), Grib2Error> {
        let min_pos = wind_layer.meridional_value_grid.grid.get_min_pos();
        let max_pos = wind_layer.meridional_value_grid.grid.get_max_pos();
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
                    let start_pos = map_tile_coords.to_position();
                    let end_pos = MapTileCoord::new(map_tile_coords.x + 1, map_tile_coords.y + 1, map_tile_coords.zoom).to_position();
                    let tile = Self::render_rectangle(wind_layer, &start_pos, &end_pos, MapTileCoord::TILE_SIZE_PX, MapTileCoord::TILE_SIZE_PX).unwrap(); // TODO
                    Self::save_tile(&tile, zoom, x, y, base_path);
                })
            }
        }

        Ok(())
    }


    pub fn render_rectangle(
        wind_layer: &DwdWindLayer,
        min_pos: &LatLon,
        max_pos: &LatLon,
        width: u32,
        height: u32
    ) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(width, height)?;

        Self::draw_color_bg_rectangle(wind_layer, &mut drawable, min_pos, max_pos);
        Self::draw_wind_arrows_rectangle(wind_layer, &mut drawable, min_pos, max_pos)?;

        return Ok(drawable);
    }


    fn draw_color_bg_rectangle(
        wind_layer: &DwdWindLayer,
        drawable: &mut Drawable,
        min_pos: &LatLon,
        max_pos: &LatLon
    ) {
        let lat_step = (max_pos.lat - min_pos.lat) / drawable.height() as f32;
        let lon_step = (max_pos.lon - min_pos.lon) / drawable.width() as f32;
        for i in 0..drawable.height() {
            let lat = min_pos.lat + i as f32 * lat_step;
            for j in 0..drawable.width() {
                let lon = min_pos.lon + j as f32 * lon_step;
                let pos = LatLon::new(lat, lon);
                let value_e_n = wind_layer.get_wind_speed_east_north_m_per_s_by_latlon(&pos);

                if value_e_n.0 != ValueGrid::MISSING_VALUE && value_e_n.1 != ValueGrid::MISSING_VALUE {
                    let abs_value = (value_e_n.0 * value_e_n.0 + value_e_n.1 * value_e_n.1).sqrt();
                    let color = Self::get_color(abs_value);

                    drawable.draw_point(j, i, color);
                }
            }
        }
    }


    fn draw_color_bg(wind_layer: &DwdWindLayer, drawable: &mut Drawable) {
        let grid_points = wind_layer.get_latlon_grid_points();

        for i in 0..grid_points.0 {
            for j in 0..grid_points.1 {
                let idx = i * grid_points.1 + j;
                let value_e_n = wind_layer.get_wind_speed_east_north_m_per_s_by_index(idx as usize);

                if value_e_n.0 != ValueGrid::MISSING_VALUE && value_e_n.1 != ValueGrid::MISSING_VALUE {
                    let abs_value = (value_e_n.0 * value_e_n.0 + value_e_n.1 * value_e_n.1).sqrt();
                    let color = Self::get_color(abs_value);

                    drawable.draw_point(j, grid_points.0 - i - 1, color);
                }
            }
        }
    }


    fn get_color(value: f32) -> [u8; 4] {
        let u8_value = (min(value, Self::MAX_VALUE_MPS) / Self::MAX_VALUE_MPS * 255.0) as u8;

        return [255, 0, 0, u8_value]; // TODO
    }


    fn draw_wind_arrows(wind_layer: &DwdWindLayer, drawable: &mut Drawable) -> Result<(), Grib2Error> {
        let grid_points = wind_layer.get_latlon_grid_points();
        let wind_arrow_service = WindArrowService::new()?;

        for i in (0..grid_points.0).step_by(Self::WIND_DIR_DIST_PX as usize) {
            for j in (0..grid_points.1).step_by(Self::WIND_DIR_DIST_PX as usize) {
                let idx = i * grid_points.1 + j;
                let (value_e, value_n) = wind_layer.get_wind_speed_east_north_m_per_s_by_index(idx as usize);

                if value_e != ValueGrid::MISSING_VALUE && value_n != ValueGrid::MISSING_VALUE && i > 0 && j > 0 {
                    let x0 = j as u32;
                    let y0 = (grid_points.0 - i - 1) as u32;
                    let rot_rad = value_n.atan2(value_e);
                    let value_kts = (value_e * value_e + value_n * value_n).sqrt() * 1.94;
                    let img = wind_arrow_service.get_arrow(value_kts)?;
                    Self::draw_single_arrow(drawable, &img, x0, y0, rot_rad);
                    //println!("x0: {}, y0: {}, value_e: {}, value_n: {}, rot_rad: {}, value_kts: {}", x0, y0, value_e, value_n, rot_rad, value_kts);
                }
            }
        }

        return Ok(());
    }


    fn draw_wind_arrows_rectangle(
        wind_layer: &DwdWindLayer,
        drawable: &mut Drawable,
        min_pos: &LatLon,
        max_pos: &LatLon
    ) -> Result<(), Grib2Error> {
        let wind_arrow_service = WindArrowService::new()?;
        let lat_step = (max_pos.lat - min_pos.lat) / drawable.height() as f32;
        let lon_step = (max_pos.lon - min_pos.lon) / drawable.width() as f32;

        for i in (0..drawable.height()).step_by(Self::WIND_DIR_DIST_PX as usize) {
            let lat = min_pos.lat + i as f32 * lat_step;
            for j in (0..drawable.width()).step_by(Self::WIND_DIR_DIST_PX as usize) {
                let lon = min_pos.lon + j as f32 * lon_step;
                let pos = LatLon::new(lat, lon);
                let (value_e, value_n) = wind_layer.get_wind_speed_east_north_m_per_s_by_latlon(&pos);

                if value_e != ValueGrid::MISSING_VALUE && value_n != ValueGrid::MISSING_VALUE && i > 0 && j > 0 {
                    let x0 = j as u32;
                    let y0 = i as u32;
                    let rot_rad = value_n.atan2(value_e);
                    let value_kts = (value_e * value_e + value_n * value_n).sqrt() * 1.94;
                    let img = wind_arrow_service.get_arrow(value_kts)?;
                    Self::draw_single_arrow(drawable, &img, x0, y0, rot_rad);
                }
            }
        }

        return Ok(());
    }


    fn draw_single_arrow(drawable: &mut Drawable, wind_arrow: &Image, x: u32, y: u32, rot_rad: f32) {
        if x + wind_arrow.width() >= drawable.width() || y + wind_arrow.height() >= drawable.height() {
            return;
        }

        let i_offset = wind_arrow.width() as f32 / 2.0;
        let j_offset = wind_arrow.height() as f32 / 2.0;

        for i in 0..wind_arrow.width() {
            for j in 0..wind_arrow.height() {
                let px_color = wind_arrow.get_pixel_color(i, j);
                if px_color[3] != 0 {
                    let i_rot = (x as f32 + (i as f32 - i_offset) * rot_rad.cos() + (j as f32 - j_offset) * rot_rad.sin()).round() as u32;
                    let j_rot = (y as f32 - (i as f32 - i_offset) * rot_rad.sin() + (j as f32 - j_offset) * rot_rad.cos()).round() as u32;
                    drawable.draw_point(i_rot, j_rot, px_color);
                }
            }
        }
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
