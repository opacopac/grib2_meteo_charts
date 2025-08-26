use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_run_finder::IconCh1ForecastRunFinder;

pub struct IconCh1ForecastRenderer;

impl IconCh1ForecastRenderer {
    pub fn create_latest_dwd_forecasts() -> Result<(), MeteoSwissError> {
        info!("rendering latest icon ch1 forecasts...");

        info!("search available forecasts...");
        let latest_ref_datetime = IconCh1ForecastRunFinder::find_latest_ref_datetime()?;
        info!("latest ref datetime found: {:?}", latest_ref_datetime);

        info!("rendering cloud & precipitation forecast...");
        // IconD2CloudPrecipRenderer::create(&latest_run)?;
        info!("finished rendering cloud & precipitation forecast");

        info!("rendering wind forecast...");
        // IconD2WindForecastRenderer::create(&latest_run)?;
        info!("finished rendering wind forecast");

        info!("rendering temperature forecast...");
        // IconD2TempForecastRenderer::create(&latest_run)?;
        info!("finished rendering temperature forecast");

        info!("rendering vertical cloud forecast...");
        // IconD2VerticalCloudForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");

        info!("rendering vertical wind forecast...");
        // IconD2VerticalWindForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");

        Ok(())
    }
}
