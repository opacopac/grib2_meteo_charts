use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_ww_layer::DwdWwLayer;

pub struct WwChartRenderer;


impl WwChartRenderer {
    pub fn render_full_chart(ww_layer: &DwdWwLayer) -> Result<Drawable, Grib2Error> {
        let dimensions = ww_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| ww_layer.get_ww_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        cloud_layer: &DwdWwLayer,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = cloud_layer.get_lat_lon_extent();

        MapTileRenderer::create_all_tiles(
            extent,
            zoom_range,
            |pos| cloud_layer.get_ww_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: f32) -> [u8; 4] {
        return match value {
            0.0 => [0, 0, 0, 0],
            1.0 => [127, 127, 127, 63],
            2.0 => [127, 127, 127, 127],
            3.0 => [127, 127, 127, 255],
            51.0 => [0, 255, 255, 63],
            53.0 => [0, 255, 255, 127],
            55.0 => [0, 255, 255, 255],
            (61.0 | 80.0) => [0, 0, 255, 63],
            (63.0 | 81.0) => [0, 0, 255, 127],
            (65.0 | 82.0) => [0, 0, 255, 255],
            95.0 => [255, 0, 0, 127],
            96.0 => [255, 0, 0, 255],
            _ => [0, 255, 0, 127]
        };
    }
}
