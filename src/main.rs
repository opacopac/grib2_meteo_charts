use meteo_grib2_renderer::dwd_chart_builder::icon_d2_forecast_builder::IconD2ForecastBuilder;

fn main() {
    env_logger::init();

    IconD2ForecastBuilder::create_latest_dwd_forecasts();
}
