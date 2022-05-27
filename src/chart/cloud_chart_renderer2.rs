use crate::chart::single_chart_renderer2::SingleChartRenderer2;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_cloud_layer2::DwdCloudLayer2;

pub struct CloudChartRenderer2;


impl CloudChartRenderer2 {
    pub fn render_full_chart(cloud_layer: DwdCloudLayer2) -> Result<Drawable, Grib2Error> {
        let dimensions = cloud_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer2::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| cloud_layer.get_cloud_cover_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [127, 127, 127, u8_value]; // TODO
    }
}
