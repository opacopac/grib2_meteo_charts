use crate::meteo_swiss::forecast_renderer::icon_ch1_temp_forecast_renderer::IconCh1TempForecastRenderer;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::forecast_search::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;


pub struct IconCh1ForecastRenderer;


impl IconCh1ForecastRenderer {
    pub fn create_latest_dwd_forecasts() -> Result<(), MeteoSwissError> {
        let model = IconChForecastModel::IconCh1;
        info!("rendering latest icon ch1 forecasts...");

        info!("search latest available forecast...");
        let latest_ref_datetime = IconChForecastSearchService::find_latest_ref_datetime(&model)?;
        info!("latest ref datetime found: {:?}", latest_ref_datetime);

        info!("search t2m forecast steps...");
        let forecast_steps_t2m = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &IconChForecastVariable::T2m,
            &latest_ref_datetime,
        )?;
        let forecast_run = IconChForecastRun::new(
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps_t2m,
        );
        info!("found {} t2m forecast steps", forecast_run.steps.len());

        info!("rendering cloud & precipitation forecast...");
        // IconD2CloudPrecipRenderer::create(&latest_run)?;
        info!("finished rendering cloud & precipitation forecast");

        info!("rendering wind forecast...");
        // IconD2WindForecastRenderer::create(&latest_run)?;
        info!("finished rendering wind forecast");

        info!("rendering temperature forecast...");
        IconCh1TempForecastRenderer::create(&forecast_run)?;
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
