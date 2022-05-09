use min_max::min;
use crate::chart::wind_arrow_service::WindArrowService;

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


    pub fn render(wind_layer: &DwdWindLayer) -> Result<Drawable, Grib2Error> {
        let grid_points = wind_layer.get_latlon_grid_points();
        let mut drawable = Drawable::create_empty(grid_points.1, grid_points.0)?;

        Self::draw_color_bg(wind_layer, &mut drawable);
        Self::draw_wind_arrows(wind_layer, &mut drawable)?;

        return Ok(drawable);
    }


    fn draw_color_bg(wind_layer: &DwdWindLayer, drawable: &mut Drawable) {
        let grid_points = wind_layer.get_latlon_grid_points();

        for i in 0..grid_points.0 {
            for j in 0..grid_points.1 {
                let idx = i * grid_points.1 + j;
                let value = wind_layer.get_wind_speed_m_per_s_by_index(idx as usize);

                if value.0 != ValueGrid::MISSING_VALUE && value.1 != ValueGrid::MISSING_VALUE {
                    let abs_value = (value.0 * value.0 + value.1 * value.1).sqrt();
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
                let value = wind_layer.get_wind_speed_m_per_s_by_index(idx as usize);

                if value.0 != ValueGrid::MISSING_VALUE && value.1 != ValueGrid::MISSING_VALUE && i > 0 && j > 0 {
                    let x0 = j as u32;
                    let y0 = (grid_points.0 - i - 1) as u32;
                    /*let x1 = x0 + value.0 as i64;
                    let y1 = y0 + value.1 as i64;
                    drawable.draw_line(x0, y0, x1, y1, [255, 255, 255, 255]);*/

                    let value_kts = (value.0 * value.0 + value.1 * value.1).sqrt() * 1.94;
                    let img = wind_arrow_service.get_arrow(value_kts)?;
                    Self::draw_single_arrow(drawable, &img, x0, y0, 0.0);
                }
            }
        }

        return Ok(());
    }


    fn draw_single_arrow(drawable: &mut Drawable, wind_arrow: &Image, x: u32, y: u32, rot: f32) {
        if x + wind_arrow.width() >= drawable.width() || y + wind_arrow.height() >= drawable.height() {
            return;
        }

        for i in 0..wind_arrow.width() {
            for j in 0..wind_arrow.height() {
                let px_color = wind_arrow.get_pixel_color(i, j);
                if px_color[3] != 0 {
                    drawable.draw_point(x + i, y + j, px_color);
                }
            }
        }
    }
}
