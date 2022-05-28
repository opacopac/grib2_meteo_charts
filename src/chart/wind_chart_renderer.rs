use min_max::min;

use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::chart::wind_arrow_service::WindArrowService;
use crate::geo::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::imaging::image::Image;
use crate::meteo_dwd::dwd_wind_layer::DwdWindLayer;

pub struct WindChartRenderer;


impl WindChartRenderer {
    const MAX_VALUE_MPS: f32 = 20.0;
    const WIND_DIR_DIST_PX: u32 = 50;


    pub fn render_full_chart(wind_layer: &DwdWindLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = wind_layer.get_grid_dimensions();
        let mut drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| wind_layer.get_wind_speed_tot_xy(x, y),
            |value| Self::color_fn(value)
        )?;
        let extent = wind_layer.get_lat_lon_extent();
        let _ = Self::draw_wind_arrows_rectangle(
            wind_layer,
            &mut drawable,
            &extent.min_coord,
            &extent.max_coord
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        wind_layer: &DwdWindLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = wind_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| wind_layer.get_wind_speed_tot_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (min(value, Self::MAX_VALUE_MPS) / Self::MAX_VALUE_MPS * 255.0) as u8;

        return [255, 0, 0, u8_value]; // TODO
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
                let value_e_n = wind_layer.get_wind_speed_e_n_by_lat_lon(&pos);
                match value_e_n {
                    Some(e_n) => {
                        if i > 0 && j > 0 {
                            let x0 = j as u32;
                            let y0 = i as u32;
                            let value_e = e_n.0;
                            let value_n = e_n.1;
                            let rot_rad = value_n.atan2(value_e);
                            let value_kts = (value_e * value_e + value_n * value_n).sqrt() * 1.94;
                            let img = wind_arrow_service.get_arrow(value_kts)?;
                            Self::draw_single_arrow(drawable, &img, x0, y0, rot_rad);
                        }
                    },
                    None => {}
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
}
