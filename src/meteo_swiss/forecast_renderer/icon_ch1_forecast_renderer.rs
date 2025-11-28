use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_chart::forecast_renderer::cloud_precip_forecast_renderer::CloudPrecipForecastRenderer;
use crate::meteo_chart::forecast_renderer::temp_2m_forecast_renderer::Temp2mForecastRenderer;
use crate::meteo_chart::forecast_renderer::vertical_clouds_forecast_renderer::VerticalCloudsForecastRenderer;
use crate::meteo_chart::forecast_renderer::vertical_wind_forecast_renderer::VerticalWindForecastRenderer;
use crate::meteo_chart::forecast_renderer::wind_10m_forecast_renderer::Wind10mForecastRenderer;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::meteo_swiss::common::icon_ch1_model_config::IconCh1ModelConfig;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_assets_service::IconChAssetsService;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_search_service::IconChForecastSearchService;
use crate::meteo_swiss::file_reader::icon_ch_clc_reader::IconChClcReader;
use crate::meteo_swiss::file_reader::icon_ch_cloud_precip_reader::IconChCloudPrecipReader;
use crate::meteo_swiss::file_reader::icon_ch_hhl_reader::IconChHhlReader;
use crate::meteo_swiss::file_reader::icon_ch_hor_const_reader::IconHorConstReader;
use crate::meteo_swiss::file_reader::icon_ch_t_2m_reader::IconChT2mReader;
use crate::meteo_swiss::file_reader::icon_ch_vertical_wind_reader::IconChVerticalWindReader;
use crate::meteo_swiss::file_reader::icon_ch_wind_10m_reader::IconChWind10mReader;
use crate::meteo_swiss::file_reader::icon_ch_ww_reader::IconChWwReader;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use log::info;


pub struct IconCh1ForecastRenderer;


const MODEL: IconChForecastModel = IconChForecastModel::IconCh1;


impl IconCh1ForecastRenderer {
    pub fn render_latest_forecasts(
        variable_filter: &[String],
        step_filter: &[usize],
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
            Self::render_cloud_precip_forecast(&step_filter, &unstructured_grid, &date_ref)?;
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
                Self::render_vertical_clouds_forecast(&step_filter, &unstructured_grid, &date_ref, &hhl_grids)?;
                info!("finished rendering vertical cloud forecast");
            }

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
                info!("rendering vertical wind forecast...");
                Self::render_vertical_wind_forecast(&step_filter, &unstructured_grid, &date_ref, &hhl_grids)?;
                info!("finished rendering vertical cloud forecast");
            }
        }

        Ok(())
    }


    fn render_cloud_precip_forecast(
        step_filter: &[usize],
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
    ) -> Result<(), MeteoSwissError> {
        let fc_run = Self::get_forecast_run(&date_ref)?;
        let fc_steps_clct = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::Clct, &date_ref, true)?;
        let fc_steps_tot_prec = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::TotPrec, &date_ref, false)?;
        let fc_steps_ceiling = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::Ceiling, &date_ref, false)?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            let cloud_precip_layer = IconChCloudPrecipReader::read_layer(fc_step, &fc_steps_clct, &fc_steps_tot_prec, unstructured_grid)?;
            let ww_layer = IconChWwReader::read_layer(fc_step, &fc_steps_clct, &fc_steps_ceiling, unstructured_grid)?;
            
            Ok((cloud_precip_layer, ww_layer))
        };

        CloudPrecipForecastRenderer::render(&fc_run, &fc_steps_clct, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_wind_10m_forecast(
        step_filter: &[usize],
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
    ) -> Result<(), MeteoSwissError> {
        let fc_run = Self::get_forecast_run(&date_ref)?;
        let fc_steps_u10m = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::U10m, &date_ref, false)?;
        let fc_steps_v10m = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::V10m, &date_ref, false)?;
        let fc_steps_vmax10m = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::VMax10m, &date_ref, false)?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            IconChWind10mReader::read_layer(fc_step, &fc_steps_u10m, &fc_steps_v10m, &fc_steps_vmax10m, unstructured_grid)
        };

        Wind10mForecastRenderer::render(&fc_run, &fc_steps_u10m, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_temp_2m_forecast(
        step_filter: &[usize],
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
    ) -> Result<(), MeteoSwissError> {
        let fc_run = Self::get_forecast_run(&date_ref)?;
        let fc_steps_t2m = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::T2m, &date_ref, false)?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            IconChT2mReader::read_layer(fc_step, &fc_steps_t2m, unstructured_grid)
        };

        Temp2mForecastRenderer::render(&fc_run, &fc_steps_t2m, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_vertical_clouds_forecast(
        step_filter: &[usize],
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
    ) -> Result<(), MeteoSwissError> {
        let fc_run = Self::get_forecast_run(&date_ref)?;
        let fc_steps_clc = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::Clc, &date_ref, false)?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            IconChClcReader::read_layer(fc_step, &fc_steps_clc, unstructured_grid, hhl_grids)
        };

        VerticalCloudsForecastRenderer::render(
            &fc_run,
            &fc_steps_clc,
            step_filter,
            read_fn,
        )?;

        Ok(())
    }


    fn render_vertical_wind_forecast(
        step_filter: &[usize],
        unstructured_grid: &UnstructuredGrid,
        date_ref: &IconChForecastReferenceDateTime,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
    ) -> Result<(), MeteoSwissError> {
        let fc_run = Self::get_forecast_run(&date_ref)?;
        let fc_steps_u = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::U, &date_ref, false)?;
        let fc_steps_v = Self::get_forecast_run_steps(&MODEL, IconChForecastVariable::V, &date_ref, false)?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            IconChVerticalWindReader::read_layer(fc_step, &fc_steps_u, &fc_steps_v, unstructured_grid, hhl_grids)
        };

        VerticalWindForecastRenderer::render(
            &fc_run,
            &fc_steps_u,
            &step_filter,
            read_fn,
        )?;

        Ok(())
    }


    fn get_forecast_run(latest_ref_datetime: &IconChForecastReferenceDateTime) -> Result<MeteoForecastRun, MeteoSwissError> {
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconCh1,
            latest_ref_datetime.get_date(),
            IconChForecastRunName::create_from_datetime(&latest_ref_datetime.datetime)?.get_name(),
        );

        Ok(fc_run)
    }


    fn get_forecast_run_steps(
        model: &IconChForecastModel,
        variable: IconChForecastVariable,
        latest_ref_datetime: &IconChForecastReferenceDateTime,
        skip_first_step: bool,
    ) -> Result<Vec<MeteoForecastRunStep>, MeteoSwissError> {
        let fc_steps = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &variable,
            &latest_ref_datetime,
        )?;

        let steps = fc_steps
            .iter()
            .enumerate()
            .filter(|(i, _step)| !skip_first_step || i >= &1)
            .map(|(i, step)| MeteoForecastRunStep::new(i, step.href.clone()))
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
