use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::dwd_layer::dwd_cloud_layer::DwdCloudLayer;

pub struct CloudChartRenderer;


impl CloudChartRenderer {
    pub fn render_full_chart(cloud_layer: &DwdCloudLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = cloud_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| cloud_layer.get_cloud_cover_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        cloud_layer: &DwdCloudLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = cloud_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| cloud_layer.get_cloud_cover_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [127, 127, 127, u8_value]; // TODO
    }
}
