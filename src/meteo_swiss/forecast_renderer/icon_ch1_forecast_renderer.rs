use crate::meteo_layer::meteo_layer::MeteoLayer;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_assets_service::IconChAssetsService;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::file_reader::icon_ch_hhl_reader::IconChHhlReader;
use crate::meteo_swiss::file_reader::icon_ch_hor_const_reader::IconHorConstReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_cloud_precip_forecast_renderer::IconCh1CloudPrecipRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_temp_forecast_renderer::IconCh1TempForecastRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch1_wind_10m_forecast_renderer::IconCh1Wind10mForecastRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch_vertical_cloud_forecast_renderer::IconCh1VerticalCloudForecastRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch_vertical_wind_forecast_renderer::IconCh1VerticalWindForecastRenderer;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;
use std::ops::RangeInclusive;

pub struct IconCh1ForecastRenderer;


const MODEL: IconChForecastModel = IconChForecastModel::IconCh1;
const VERTICAL_LEVEL_RANGE: RangeInclusive<usize> = 25..=65;


impl IconCh1ForecastRenderer {
    pub fn render_latest_forecasts(
        variable_filter: &Vec<String>,
        step_filter: &Vec<usize>,
    ) -> Result<(), MeteoSwissError> {
        info!("rendering latest icon ch1 forecasts...");

        info!("reading horizontal/vertical constants...");
        let icon_ch1_assets = IconChAssetsService::get()?;

        let hor_consts = icon_ch1_assets.get_horizontal_constants().unwrap();
        let unstructured_grid = IconHorConstReader::read_grid_from_file(&hor_consts.href)?;

        let vert_consts = icon_ch1_assets.get_vertical_constants().unwrap();
        let hhl_grids = IconChHhlReader::read_grids(&vert_consts.href, &unstructured_grid, Some(VERTICAL_LEVEL_RANGE))?;
        info!("finished reading horizontal/vertical constants");

        info!("search latest available forecast...");
        let date_ref = IconChForecastSearchService::find_latest_ref_datetime(&MODEL)?;
        info!("latest ref datetime found: {:?}", date_ref);

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayer::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            let fc_run_clct = Self::get_forecast_run(&MODEL, IconChForecastVariable::Clct, &date_ref)?;
            let fc_run_tot_prec = Self::get_forecast_run(&MODEL, IconChForecastVariable::TotPrec, &date_ref)?;
            IconCh1CloudPrecipRenderer::render(&fc_run_clct, &fc_run_tot_prec, &unstructured_grid, &step_filter)?;
            info!("finished rendering cloud & precipitation forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayer::Wind10m.get_name()) {
            info!("rendering wind 10m forecast...");
            let fc_run_u10m = Self::get_forecast_run(&MODEL, IconChForecastVariable::U10m, &date_ref)?;
            let fc_run_v10m = Self::get_forecast_run(&MODEL, IconChForecastVariable::V10m, &date_ref)?;
            let fc_run_vmax10m = Self::get_forecast_run(&MODEL, IconChForecastVariable::VMax10m, &date_ref)?;
            IconCh1Wind10mForecastRenderer::render(&fc_run_u10m, &fc_run_v10m, &fc_run_vmax10m, &unstructured_grid, &step_filter)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayer::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            let fc_run_t2m = Self::get_forecast_run(&MODEL, IconChForecastVariable::T2m, &date_ref)?;
            IconCh1TempForecastRenderer::render(&fc_run_t2m, &unstructured_grid, &step_filter)?;
            info!("finished rendering temperature 2m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayer::VerticalCloud.get_name()) {
            info!("rendering vertical cloud forecast...");
            let fc_run_clc = Self::get_forecast_run(&MODEL, IconChForecastVariable::Clc, &date_ref)?;
            IconCh1VerticalCloudForecastRenderer::render(
                &fc_run_clc,
                &unstructured_grid,
                &hhl_grids,
                &step_filter,
            )?;
            info!("finished rendering vertical cloud forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayer::VerticalWind.get_name()) {
            info!("rendering vertical wind forecast...");
            let fc_run_u = Self::get_forecast_run(&MODEL, IconChForecastVariable::U, &date_ref)?;
            let fc_run_v = Self::get_forecast_run(&MODEL, IconChForecastVariable::V, &date_ref)?;
            IconCh1VerticalWindForecastRenderer::render(
                &fc_run_u,
                &fc_run_v,
                &unstructured_grid,
                &hhl_grids,
                &step_filter,
            )?;
            info!("finished rendering vertical cloud forecast");
        }

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


#[cfg(test)]
mod tests {
    use crate::meteo_layer::meteo_layer::MeteoLayer;
    use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer::IconCh1ForecastRenderer;

    #[test]
    fn it_successfully_renders_a_part_of_the_latest_icon_ch1_forecasts() {
        // given
        let variable_filter = vec![MeteoLayer::Temp2m.get_name()];
        let step_filter = vec![2, 3, 4];
        
        // when
        let result = IconCh1ForecastRenderer::render_latest_forecasts(&variable_filter, &step_filter);
        
        // then
        assert!(result.is_ok());
    }
}