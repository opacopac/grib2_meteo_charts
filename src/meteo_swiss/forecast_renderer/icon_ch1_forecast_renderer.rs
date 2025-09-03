use crate::meteo_swiss::file_reader::icon_ch_hor_const_reader::IconHorConstReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_cloud_precip_forecast_renderer::IconCh1CloudPrecipRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_temp_forecast_renderer::IconCh1TempForecastRenderer;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::forecast_search::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;

pub const HOR_CONST_TEST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2";


pub struct IconCh1ForecastRenderer;


impl IconCh1ForecastRenderer {
    pub fn create_latest_forecasts() -> Result<(), MeteoSwissError> {
        let model = IconChForecastModel::IconCh1;
        info!("rendering latest icon ch1 forecasts...");

        info!("reading horizontal constants...");
        let unstructured_grid = IconHorConstReader::read_grid_from_file(HOR_CONST_TEST_FILE)?;
        info!("finished reading horizontal constants");

        info!("search latest available forecast...");
        let latest_ref_datetime = IconChForecastSearchService::find_latest_ref_datetime(&model)?;
        info!("latest ref datetime found: {:?}", latest_ref_datetime);

        info!("rendering cloud & precipitation forecast...");
        let forecast_run_clct = Self::get_forecast_run_clct(&model, &latest_ref_datetime)?;
        let forecast_run_tot_prec = Self::get_forecast_run_tot_prec(&model, &latest_ref_datetime)?;
        IconCh1CloudPrecipRenderer::create(&forecast_run_clct, &forecast_run_tot_prec, &unstructured_grid)?;
        info!("finished rendering cloud & precipitation forecast");

        /*info!("rendering wind forecast...");
        IconD2WindForecastRenderer::create(&latest_run)?;
        info!("finished rendering wind forecast");*/

        info!("rendering temperature forecast...");
        let forecast_run_t2m = Self::get_forecast_run_temp(&model, &latest_ref_datetime)?;
        IconCh1TempForecastRenderer::create(&forecast_run_t2m, &unstructured_grid)?;
        info!("finished rendering temperature forecast");

        /*info!("rendering vertical cloud forecast...");
        IconD2VerticalCloudForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");*/

        /*info!("rendering vertical wind forecast...");
        IconD2VerticalWindForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");*/

        Ok(())
    }


    fn get_forecast_run_clct(model: &IconChForecastModel, latest_ref_datetime: &IconChForecastReferenceDateTime) -> Result<IconChForecastRun, MeteoSwissError> {
        let forecast_steps_clct = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &IconChForecastVariable::Clct,
            &latest_ref_datetime,
        )?;
        let forecast_run_clct = IconChForecastRun::new(
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps_clct,
        );

        Ok(forecast_run_clct)
    }


    fn get_forecast_run_tot_prec(model: &IconChForecastModel, latest_ref_datetime: &IconChForecastReferenceDateTime) -> Result<IconChForecastRun, MeteoSwissError> {
        let forecast_steps_tot_prec = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &IconChForecastVariable::TotPrec,
            &latest_ref_datetime,
        )?;
        let forecast_run_tot_prec = IconChForecastRun::new(
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps_tot_prec,
        );

        Ok(forecast_run_tot_prec)
    }


    fn get_forecast_run_temp(model: &IconChForecastModel, latest_ref_datetime: &IconChForecastReferenceDateTime) -> Result<IconChForecastRun, MeteoSwissError> {
        let forecast_steps_t2m = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &IconChForecastVariable::T2m,
            &latest_ref_datetime,
        )?;
        let forecast_run_t2m = IconChForecastRun::new(
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps_t2m,
        );

        Ok(forecast_run_t2m)
    }
}
