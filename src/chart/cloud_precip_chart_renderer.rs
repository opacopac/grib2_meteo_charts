use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_cloud_precip_layer::DwdCloudPrecipLayer;

pub struct CloudPrecipChartRenderer;


impl CloudPrecipChartRenderer {
    pub fn render_full_chart(cloud_precip_layer: &DwdCloudPrecipLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = cloud_precip_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| cloud_precip_layer.get_cloud_and_precip_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        cloud_layer: &DwdCloudPrecipLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = cloud_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| cloud_layer.get_cloud_and_precip_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: (f32, f32)) -> [u8; 4] {
        if value.1 > 0.0 {
            let u8_value = (value.1 * 255.0).floor() as u8;
            return [0, 0, 255, u8_value]; // TODO
        }

        let u8_value = (value.0 * 255.0).floor() as u8;

        return [127, 127, 127, u8_value]; // TODO
    }
}
