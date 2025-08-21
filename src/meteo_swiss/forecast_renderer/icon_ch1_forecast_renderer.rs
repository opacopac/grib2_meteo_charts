use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;

pub struct IconCh1ForecastRenderer;

impl IconCh1ForecastRenderer {
    pub fn create_latest_dwd_forecasts() -> Result<(), MeteoSwissError> {
        info!("creating latest dwd forecasts...");

        info!("search available forecasts...");
        // let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run()?;
        // info!("latest run found: {:?}", &latest_run);

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
