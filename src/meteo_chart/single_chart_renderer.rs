use crate::dwd::value_grid::ValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;

pub struct SingleChartRenderer;

impl SingleChartRenderer {
    pub fn create(
        value_grid: &ValueGrid,
        color_fn: fn(f32) -> [u8; 4]
    ) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(value_grid.grid.lon_grid_points, value_grid.grid.lat_grid_points)?;

        for i in 0..value_grid.grid.lat_grid_points {
            for j in 0..value_grid.grid.lon_grid_points {
                let idx = i * value_grid.grid.lon_grid_points + j;
                let value = value_grid.get_value_by_index(idx as usize);

                if value != ValueGrid::MISSING_VALUE {
                    let color = color_fn(value);

                    drawable.draw_point(j, value_grid.grid.lat_grid_points - i - 1, color);
                }
            }
        }

        return Ok(drawable);
    }
}
