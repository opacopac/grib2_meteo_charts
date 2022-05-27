use crate::chart::map_tile_renderer2::MapTileRenderer2;
use crate::chart::single_chart_renderer2::SingleChartRenderer2;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_precip_layer2::DwdPrecipLayer2;

pub struct PrecipChartRenderer2;


impl PrecipChartRenderer2 {
    pub fn render_full_chart(precip_layer: &DwdPrecipLayer2) -> Result<Drawable, Grib2Error> {
        let dimensions = precip_layer.get_grid_dimensions();
        let drawable = SingleChartRenderer2::render(
            dimensions.0 as u32,
            dimensions.1 as u32,
            |x, y| precip_layer.get_total_precipitation_by_xy(x, y),
            |value| Self::color_fn(value)
        )?;

        return Ok(drawable);
    }


    pub fn render_map_tiles<S>(
        precip_layer: DwdPrecipLayer2,
        zoom_range: (u32, u32),
        save_fn: S
    ) -> Result<(), Grib2Error> where
        S: Fn(&Drawable, u32, u32, u32) -> () + Sync
    {
        let extent = precip_layer.get_lat_lon_extent();

        MapTileRenderer2::create_all_tiles(
            extent,
            zoom_range,
            |pos| precip_layer.get_total_precipitation_by_lat_lon(pos),
            |value| Self::color_fn(value),
            save_fn
        )
    }


    fn color_fn(value: f32) -> [u8; 4] {
        let u8_value = (value  * 255.0).floor() as u8;

        return [0, 127, 255, u8_value]; // TODO
    }
}
