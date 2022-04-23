use crate::dwd::cloud_cover::cloud_cover_layer::CloudCoverLayer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::COLOR_TRANSPARENT;
use crate::imaging::drawable::Drawable;

pub struct CloudCoverChartRenderer;

impl CloudCoverChartRenderer {
    pub fn create(layer: CloudCoverLayer) -> Result<Drawable, Grib2Error> {
        let mut drawable = Drawable::create_empty(layer.lon_grid_points(), layer.lat_grid_points())?;

        for i in 0..layer.lat_grid_points() {
            for j in 0..layer.lon_grid_points() {
                let idx = i * layer.lon_grid_points() + j;
                let value = layer.get_value_by_index(idx as usize);

                if value != CloudCoverLayer::MISSING_VALUE {
                    //let color_value = (value / 100.0 * 255.0).floor() as u8;
                    //let color = [color_value, color_value, color_value, 255];
                    let color = [255, 0, 0, 255];

                    drawable.draw_point(j, i, color);
                }
            }
        }

        return Ok(drawable);
    }
}
