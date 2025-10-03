use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_cloud_precip_forecast_renderer::IconD2CloudPrecipRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd::dwd_forecast_renderer::icon_d2_temp_forecast_renderer::IconD2TempForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_cloud_forecast_renderer::IconD2VerticalCloudForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_wind_forecast_renderer::IconD2VerticalWindForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_wind_10m_forecast_renderer::IconD2Wind10mForecastRenderer;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use log::info;


pub struct IconD2ForecastRenderer;


impl IconD2ForecastRenderer {
    pub fn render_latest_forecasts(
        variable_filter: &Vec<String>,
        step_filter: &Vec<usize>,
    ) -> Result<(), ForecastRendererError> {
        info!("creating latest dwd forecasts...");

        info!("search available forecasts...");
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run()?;
        info!("latest run found: {:?}", &latest_run);

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            IconD2CloudPrecipRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering cloud & precipitation forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Wind10m.get_name()) {
            info!("rendering wind 10m forecast...");
            IconD2Wind10mForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            IconD2TempForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering temperature 2m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) {
            info!("rendering vertical cloud forecast...");
            IconD2VerticalCloudForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering vertical cloud forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
            info!("rendering vertical wind forecast...");
            IconD2VerticalWindForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering vertical cloud forecast");
        }

        Ok(())
    }
}
