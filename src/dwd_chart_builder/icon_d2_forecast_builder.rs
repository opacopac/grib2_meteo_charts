use log::info;

use crate::dwd_chart_builder::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd_chart_builder::icon_d2_vertical_cloud_chart_builder::IconD2VerticalCloudChartBuilder;
use crate::dwd_chart_builder::icon_d2_weather_chart_builder::IconD2WeatherChartBuilder;
use crate::dwd_chart_builder::icon_d2_wind_chart_builder::IconD2WindChartBuilder;

pub struct IconD2ForecastBuilder;


impl IconD2ForecastBuilder {
    pub fn create_latest_dwd_forecasts() {
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run().unwrap(); // TODO
        info!("latest run found: {:?}", &latest_run);

        IconD2WeatherChartBuilder::create(&latest_run);
        IconD2WindChartBuilder::create(&latest_run);
        IconD2VerticalCloudChartBuilder::create(&latest_run);
    }
}
