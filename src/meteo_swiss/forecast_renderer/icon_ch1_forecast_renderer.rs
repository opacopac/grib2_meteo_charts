use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_run_finder::IconCh1ForecastRunFinder;
use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_urls_finder::IconCh1ForecastUrlsFinder;

pub struct IconCh1ForecastRenderer;

impl IconCh1ForecastRenderer {
    pub fn create_latest_dwd_forecasts() -> Result<(), MeteoSwissError> {
        info!("rendering latest icon ch1 forecasts...");

        info!("search latest available forecast...");
        let latest_ref_datetime = IconCh1ForecastRunFinder::find_latest_ref_datetime()?;
        info!("latest ref datetime found: {:?}", latest_ref_datetime);

        info!("search t2m forecast steps...");
        let forecast_steps_t2m = IconCh1ForecastUrlsFinder::find_forecast_file_urls(
            crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable::T2m,
            latest_ref_datetime,
        );
        info!("found {} t2m forecast steps", forecast_steps_t2m.as_ref().map(|s| s.len()).unwrap_or(0));

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
