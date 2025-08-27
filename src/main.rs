extern crate core;

use meteo_grib2_renderer::dwd::dwd_forecast_renderer::icon_d2_forecast_renderer::IconD2ForecastRenderer;
use meteo_grib2_renderer::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer::IconCh1ForecastRenderer;

fn main() {
    env_logger::init();

    //let _ = IconD2ForecastRenderer::create_latest_forecasts().unwrap();
    let _ = IconCh1ForecastRenderer::create_latest_forecasts().unwrap();
}
