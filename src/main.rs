use meteo_grib2_renderer::dwd_chart_builder::icon_d2_chart_builder::IconD2ChartBuilder;

fn main() {
    env_logger::init();

    IconD2ChartBuilder::create_latest_dwd_forecasts();
}
