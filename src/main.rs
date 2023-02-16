use meteo_grib2_renderer::dwd_forecast_renderer::icon_d2_forecast_renderer::IconD2ForecastRenderer;

fn main() {
    env_logger::init();

    IconD2ForecastRenderer::create_latest_dwd_forecasts();
}
