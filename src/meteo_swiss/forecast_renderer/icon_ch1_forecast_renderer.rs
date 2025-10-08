use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::forecast_renderer::temp_2m_forecast_renderer::Temp2mForecastRenderer;
use crate::meteo_chart::forecast_renderer::wind_10m_forecast_renderer::Wind10mForecastRenderer;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_assets_service::IconChAssetsService;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::file_reader::icon_ch_hhl_reader::IconChHhlReader;
use crate::meteo_swiss::file_reader::icon_ch_hor_const_reader::IconHorConstReader;
use crate::meteo_swiss::file_reader::icon_ch_t_2m_reader::IconChT2mReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_u_10m_reader::IconChWindU10mReader;
use crate::meteo_swiss::forecast_renderer::icon_ch1_cloud_precip_forecast_renderer::IconCh1CloudPrecipRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch_vertical_cloud_forecast_renderer::IconCh1VerticalCloudForecastRenderer;
use crate::meteo_swiss::forecast_renderer::icon_ch_vertical_wind_forecast_renderer::IconCh1VerticalWindForecastRenderer;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use log::info;


pub struct IconCh1ForecastRenderer;


const MODEL: IconChForecastModel = IconChForecastModel::IconCh1;


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
        info!("finished reading horizontal/vertical constants");

        info!("search latest available forecast...");
        let date_ref = IconChForecastSearchService::find_latest_ref_datetime(&MODEL)?;
        info!("latest ref datetime found: {:?}", date_ref);

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            let fc_run_clct = Self::get_forecast_run(&MODEL, IconChForecastVariable::Clct, &date_ref)?;
            let fc_run_tot_prec = Self::get_forecast_run(&MODEL, IconChForecastVariable::TotPrec, &date_ref)?;
            let fc_run_ceiling = Self::get_forecast_run(&MODEL, IconChForecastVariable::Ceiling, &date_ref)?;
            IconCh1CloudPrecipRenderer::render(&fc_run_clct, &fc_run_tot_prec, &fc_run_ceiling, &unstructured_grid, &step_filter)?;
            info!("finished rendering cloud & precipitation forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Wind10m.get_name()) {
            info!("rendering wind 10m forecast...");
            Self::render_wind_10m_forecast(&step_filter, &unstructured_grid, &date_ref)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            Self::render_temp_2m_forecast(&step_filter, &unstructured_grid, &date_ref)?;
            info!("finished rendering temperature 2m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name())
            || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()
        ) {
            info!("reading hhl grids...");
            let vertical_levels = IconCh1ModelConfig::get_vertical_level_range();
            let hhl_grids = IconChHhlReader::read_grids(&vert_consts.href, &unstructured_grid, Some(&vertical_levels))?;
            info!("finished reading hhl grids");

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) {
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

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
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
        }

        Ok(())
    }


    fn render_wind_10m_forecast(
        step_filter: &Vec<usize>,
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
    ) -> Result<(), MeteoSwissError> {
        let fc_run_u10m = Self::get_forecast_run2(&MODEL, IconChForecastVariable::U10m, &date_ref)?;
        let fc_steps_u10m = Self::get_forecast_run2_steps(&MODEL, IconChForecastVariable::U10m, &date_ref)?;
        let fc_steps_v10m = Self::get_forecast_run2_steps(&MODEL, IconChForecastVariable::V10m, &date_ref)?;
        let fc_steps_vmax10m = Self::get_forecast_run2_steps(&MODEL, IconChForecastVariable::VMax10m, &date_ref)?;
        let read_fn = |step: &MeteoForecastRun2Step| {
            let u10m_grid = IconChWindU10mReader::read_grid_from_file(&step.get_file_url(), &unstructured_grid)?;
            let v10m_step = &fc_steps_v10m[step.get_step_nr()];
            let v10m_grid = IconChWindU10mReader::read_grid_from_file(&v10m_step.get_file_url(), &unstructured_grid)?;
            let vmax10m_step = &fc_steps_vmax10m[step.get_step_nr()];
            let vmax10m_grid = IconChWindU10mReader::read_grid_from_file(&vmax10m_step.get_file_url(), &unstructured_grid)?;

            MeteoWind10mLayer::new(u10m_grid, v10m_grid, Some(vmax10m_grid))
        };

        Wind10mForecastRenderer::render(&fc_run_u10m, &fc_steps_u10m, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_temp_2m_forecast(
        step_filter: &Vec<usize>,
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
    ) -> Result<(), MeteoSwissError> {
        let fc_run_t2m = Self::get_forecast_run2(&MODEL, IconChForecastVariable::T2m, &date_ref)?;
        let fc_steps_t2m = Self::get_forecast_run2_steps(&MODEL, IconChForecastVariable::T2m, &date_ref)?;
        let read_fn = |step: &MeteoForecastRun2Step| {
            IconChT2mReader::read_layer_from_file(&step.get_file_url(), &unstructured_grid)
        };

        Temp2mForecastRenderer::render(&fc_run_t2m, &fc_steps_t2m, &step_filter, read_fn)?;

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
            model.clone(),
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps,
        );

        Ok(forecast_run)
    }


    fn get_forecast_run2(
        model: &IconChForecastModel,
        variable: IconChForecastVariable,
        latest_ref_datetime: &IconChForecastReferenceDateTime,
    ) -> Result<MeteoForecastRun2, MeteoSwissError> {
        let forecast_steps = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &variable,
            &latest_ref_datetime,
        )?;
        let forecast_run = IconChForecastRun::new(
            model.clone(),
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?,
            forecast_steps,
        );

        let forecast_run2 = MeteoForecastRun2::new(
            MeteoForecastModel::IconCh1,
            forecast_run.get_start_date(),
            forecast_run.get_name(),
        );

        Ok(forecast_run2)
    }


    fn get_forecast_run2_steps(
        model: &IconChForecastModel,
        variable: IconChForecastVariable,
        latest_ref_datetime: &IconChForecastReferenceDateTime,
    ) -> Result<Vec<MeteoForecastRun2Step>, MeteoSwissError> {
        let forecast_steps = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &variable,
            &latest_ref_datetime,
        )?;

        let steps = forecast_steps
            .iter()
            .enumerate()
            .map(|(i, step)| MeteoForecastRun2Step::new(i, step.href.clone()))
            .collect();

        Ok(steps)
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
    use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_renderer::IconCh1ForecastRenderer;

    #[test]
    fn it_successfully_renders_a_part_of_the_latest_icon_ch1_forecasts() {
        // given
        let variable_filter = vec![MeteoLayerType::Temp2m.get_name()];
        let step_filter = vec![2, 3, 4];

        // when
        let result = IconCh1ForecastRenderer::render_latest_forecasts(&variable_filter, &step_filter);

        // then
        assert!(result.is_ok());
    }
}