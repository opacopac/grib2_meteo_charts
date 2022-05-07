use min_max::min;

use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo::wind_layer::WindLayer;
use crate::meteo_dwd::dwd_wind_layer::DwdWindLayer;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct WindChartRenderer;

impl WindChartRenderer {
    const MAX_VALUE_MPS: f32 = 20.0;

    pub fn render(wind_layer: DwdWindLayer) -> Result<Drawable, Grib2Error> {
        let grid_points = wind_layer.get_latlon_grid_points();
        let mut drawable = Drawable::create_empty(grid_points.1, grid_points.0)?;

        for i in 0..grid_points.0 {
            for j in 0..grid_points.1 {
                let idx = i * grid_points.1 + j;
                let value = wind_layer.get_wind_speed_m_per_s_by_index(idx as usize);

                if value.0 != ValueGrid::MISSING_VALUE && value.1 != ValueGrid::MISSING_VALUE {
                    let abs_value = (value.0 * value.0 + value.1 * value.1).sqrt();
                    let color = Self::color_fn(abs_value);

                    drawable.draw_point(j, grid_points.0 - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (min(value, Self::MAX_VALUE_MPS) / Self::MAX_VALUE_MPS * 255.0) as u8;

        return [255, 0, 0, u8_value]; // TODO
    }
}
