use log::info;

use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_cloud_precip_forecast_renderer::IconD2CloudPrecipRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd::dwd_forecast_renderer::icon_d2_temp_forecast_renderer::IconD2TempForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_cloud_forecast_renderer::IconD2VerticalCloudForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_wind_forecast_renderer::IconD2VerticalWindForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_wind_10m_forecast_renderer::IconD2Wind10mForecastRenderer;

pub struct IconD2ForecastRenderer;


impl IconD2ForecastRenderer {
    pub fn create_latest_forecasts() -> Result<(), ForecastRendererError> {
        info!("creating latest dwd forecasts...");

        info!("search available forecasts...");
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run()?;
        info!("latest run found: {:?}", &latest_run);

        info!("rendering cloud & precipitation forecast...");
        IconD2CloudPrecipRenderer::create(&latest_run)?;
        info!("finished rendering cloud & precipitation forecast");

        info!("rendering wind forecast...");
        IconD2Wind10mForecastRenderer::create(&latest_run)?;
        info!("finished rendering wind forecast");

        info!("rendering temperature forecast...");
        IconD2TempForecastRenderer::create(&latest_run)?;
        info!("finished rendering temperature forecast");

        info!("rendering vertical cloud forecast...");
        IconD2VerticalCloudForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");

        info!("rendering vertical wind forecast...");
        IconD2VerticalWindForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");

        Ok(())
    }
}
