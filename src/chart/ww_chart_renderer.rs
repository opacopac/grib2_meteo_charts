use crate::chart::map_tile_renderer::MapTileRenderer;
use crate::chart::single_chart_renderer::SingleChartRenderer;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::imaging::drawable::Drawable;
use crate::meteo_dwd::dwd_ww_layer::DwdWwLayer;
use crate::meteo_dwd::weather_interpretation::WeatherInterpretation;

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


    fn color_fn(value: WeatherInterpretation) -> [u8; 4] {
        return match value {
            WeatherInterpretation::ClearSky => [0, 0, 0, 0],
            WeatherInterpretation::MainlyClearSky => [127, 127, 127, 63],
            WeatherInterpretation::PartlyCloudy => [127, 127, 127, 127],
            WeatherInterpretation::Overcast => [127, 127, 127, 190],
            WeatherInterpretation::Fog | WeatherInterpretation::FogDepositingRime => [127, 127, 127, 255],
            WeatherInterpretation::SlightDrizzle => [0, 255, 255, 63],
            WeatherInterpretation::ModerateDrizzle => [0, 255, 255, 127],
            WeatherInterpretation::HeavyDrizzle => [0, 255, 255, 255],
            WeatherInterpretation::SlightRainNotFreezing | WeatherInterpretation::RainShowerSlight => [0, 0, 255, 63],
            WeatherInterpretation::ModerateRainNotFreezing | WeatherInterpretation::RainShowerModerateOrHeavy => [0, 0, 255, 127],
            WeatherInterpretation::HeavyRainNotFreezing | WeatherInterpretation::RainShowerViolent => [0, 0, 255, 255],
            WeatherInterpretation::ThunderstormSlightOrModerate => [255, 0, 0, 127],
            WeatherInterpretation::ThunderstormWithHailOrHeavyThunderstorm => [255, 0, 0, 255],
            _ => { print!("{:?} ", value); return [0, 255, 0, 127]; }
        };
    }
}
