use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::dwd_layer::dwd_weather_layer::DwdWeatherLayer;
use crate::dwd_layer::dwd_weather_interpretation::DwdWeatherInterpretation;

pub struct WwChartRenderer;


impl WwChartRenderer {
    pub fn render_full_chart(ww_layer: &DwdWeatherLayer) -> Result<Drawable, Grib2Error> {
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
        cloud_layer: &DwdWeatherLayer,
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


    fn color_fn(value: DwdWeatherInterpretation) -> [u8; 4] {
        return match value {
            DwdWeatherInterpretation::ClearSky => [0, 0, 0, 0],
            DwdWeatherInterpretation::MainlyClearSky => [127, 127, 127, 63],
            DwdWeatherInterpretation::PartlyCloudy => [127, 127, 127, 127],
            DwdWeatherInterpretation::Overcast => [127, 127, 127, 190],
            DwdWeatherInterpretation::Fog | DwdWeatherInterpretation::FogDepositingRime => [127, 127, 127, 255],
            DwdWeatherInterpretation::SlightDrizzle => [0, 255, 255, 63],
            DwdWeatherInterpretation::ModerateDrizzle => [0, 255, 255, 127],
            DwdWeatherInterpretation::HeavyDrizzle => [0, 255, 255, 255],
            DwdWeatherInterpretation::SlightRainNotFreezing | DwdWeatherInterpretation::RainShowerSlight => [0, 0, 255, 63],
            DwdWeatherInterpretation::ModerateRainNotFreezing | DwdWeatherInterpretation::RainShowerModerateOrHeavy => [0, 0, 255, 127],
            DwdWeatherInterpretation::HeavyRainNotFreezing | DwdWeatherInterpretation::RainShowerViolent => [0, 0, 255, 255],
            DwdWeatherInterpretation::ThunderstormSlightOrModerate => [255, 0, 0, 127],
            DwdWeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm => [255, 0, 0, 255],
            _ => { print!("{:?} ", value); return [0, 255, 0, 127]; }
        };
    }
}
