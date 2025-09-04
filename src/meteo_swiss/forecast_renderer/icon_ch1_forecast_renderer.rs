use crate::meteo_swiss::file_reader::icon_ch_hor_const_reader::IconHorConstReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_cloud_precip_forecast_renderer::IconCh1CloudPrecipRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_temp_forecast_renderer::IconCh1TempForecastRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_wind_10m_forecast_renderer::IconCh1Wind10mForecastRenderer;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::forecast_search::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;

pub const HOR_CONST_FILE: &str = "./tests/resources/horizontal_constants_icon-ch1-eps.grib2"; // TODO


pub struct IconCh1ForecastRenderer;


impl IconCh1ForecastRenderer {
    pub fn create_latest_forecasts(
        _variables: &Vec<String>
    ) -> Result<(), MeteoSwissError> {
        let model = IconChForecastModel::IconCh1;
        info!("rendering latest icon ch1 forecasts...");

        info!("reading horizontal constants...");
        let unstructured_grid = IconHorConstReader::read_grid_from_file(HOR_CONST_FILE)?;
        info!("finished reading horizontal constants");

        info!("search latest available forecast...");
        let date_ref = IconChForecastSearchService::find_latest_ref_datetime(&model)?;
        info!("latest ref datetime found: {:?}", date_ref);

        info!("rendering cloud & precipitation forecast...");
        let fc_run_clct = Self::get_forecast_run(&model, IconChForecastVariable::Clct, &date_ref)?;
        let fc_run_tot_prec = Self::get_forecast_run(&model, IconChForecastVariable::TotPrec, &date_ref)?;
        IconCh1CloudPrecipRenderer::create(&fc_run_clct, &fc_run_tot_prec, &unstructured_grid)?;
        info!("finished rendering cloud & precipitation forecast");

        info!("rendering wind forecast...");
        let fc_run_u10m = Self::get_forecast_run(&model, IconChForecastVariable::U10m, &date_ref)?;
        let fc_run_v10m = Self::get_forecast_run(&model, IconChForecastVariable::V10m, &date_ref)?;
        let fc_run_vmax10m = Self::get_forecast_run(&model, IconChForecastVariable::VMax10m, &date_ref)?;
        IconCh1Wind10mForecastRenderer::create(&fc_run_u10m, &fc_run_v10m, &fc_run_vmax10m, &unstructured_grid)?;
        info!("finished rendering wind forecast");

        info!("rendering temperature forecast...");
        let fc_run_t2m = Self::get_forecast_run(&model, IconChForecastVariable::T2m, &date_ref)?;
        IconCh1TempForecastRenderer::create(&fc_run_t2m, &unstructured_grid)?;
        info!("finished rendering temperature forecast");

        /*info!("rendering vertical cloud forecast...");
        IconD2VerticalCloudForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");*/

        /*info!("rendering vertical wind forecast...");
        IconD2VerticalWindForecastRenderer::create(&latest_run)?;
        info!("finished rendering vertical cloud forecast");*/

        Ok(())
    }


    fn get_forecast_run(
        model: &IconChForecastModel,
        variable: IconChForecastVariable,
        latest_ref_datetime: &IconChForecastReferenceDateTime,
    ) -> Result<IconChForecastRun, MeteoSwissError> {
        let forecast_steps = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &variable,
            &latest_ref_datetime,
        )?;
        let forecast_run = IconChForecastRun::new(
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps,
        );

        Ok(forecast_run)
    }
}
