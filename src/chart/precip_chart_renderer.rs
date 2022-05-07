use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo::precip_layer::PrecipLayer;
use crate::meteo_dwd::dwd_precip_layer::DwdPrecipLayer;
use crate::meteo_dwd::value_grid::ValueGrid;

pub struct PrecipChartRenderer;

impl PrecipChartRenderer {
    pub fn render(precip_layer: &DwdPrecipLayer) -> Result<Drawable, Grib2Error> {
        let grid_points = precip_layer.get_latlon_grid_points();
        let mut drawable = Drawable::create_empty(grid_points.1, grid_points.0)?;

        for i in 0..grid_points.0 {
            for j in 0..grid_points.1 {
                let idx = i * grid_points.1 + j;
                let value = precip_layer.get_precip_rate_kg_per_m2_per_s_by_index(idx as usize);

                if value != ValueGrid::MISSING_VALUE {
                    let color = Self::color_fn(value);

                    drawable.draw_point(j, grid_points.0 - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [0, 127, 255, u8_value]; // TODO
    }
}
