use log::info;

use crate::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd_forecast_renderer::icon_d2_vertical_cloud_forecast_renderer::IconD2VerticalCloudForecastRenderer;
use crate::dwd_forecast_renderer::icon_d2_cloud_precip_forecast_renderer::IconD2CloudPrecipRenderer;
use crate::dwd_forecast_renderer::icon_d2_wind_forecast_renderer::IconD2WindForecastRenderer;

pub struct IconD2ForecastRenderer;


impl IconD2ForecastRenderer {
    pub fn create_latest_dwd_forecasts() {
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run().unwrap(); // TODO
        info!("latest run found: {:?}", &latest_run);

        IconD2CloudPrecipRenderer::create(&latest_run);
        IconD2WindForecastRenderer::create(&latest_run);
        IconD2VerticalCloudForecastRenderer::create(&latest_run);
    }
}
