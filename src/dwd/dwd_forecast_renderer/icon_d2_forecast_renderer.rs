use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::dwd_icon_clc_reader::DwdIconClcReader;
use crate::dwd::dwd_file_reader::dwd_icon_cloud_precip_reader::DwdIconCloudPrecipReader;
use crate::dwd::dwd_file_reader::dwd_icon_hhl_reader::DwdIconHhlReader;
use crate::dwd::dwd_file_reader::dwd_icon_t_2m_reader::DwdIconT2mReader;
use crate::dwd::dwd_file_reader::dwd_icon_u_reader::DwdIconUReader;
use crate::dwd::dwd_file_reader::dwd_icon_v_reader::DwdIconVReader;
use crate::dwd::dwd_file_reader::dwd_icon_weather_reader::DwdIconWeatherReader;
use crate::dwd::dwd_file_reader::dwd_icon_wind_10m_reader::DwdIconWind10mReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::meteo_chart::forecast_renderer::cloud_precip_forecast_renderer::CloudPrecipForecastRenderer;
use crate::meteo_chart::forecast_renderer::temp_2m_forecast_renderer::Temp2mForecastRenderer;
use crate::meteo_chart::forecast_renderer::vertical_clouds_forecast_renderer::VerticalCloudsForecastRenderer;
use crate::meteo_chart::forecast_renderer::vertical_wind_forecast_renderer::VerticalWindForecastRenderer;
use crate::meteo_chart::forecast_renderer::wind_10m_forecast_renderer::Wind10mForecastRenderer;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use log::info;
use std::ops::RangeInclusive;


pub struct IconD2ForecastRenderer;


impl IconD2ForecastRenderer {
    pub fn render_latest_forecasts(
        variable_filter: &[String],
        step_filter: &[usize],
    ) -> Result<(), ForecastRendererError> {
        info!("creating latest dwd forecasts...");

        info!("search available forecasts...");
        let fc_run = IconD2ForecastRunFinder::find_latest_forecast_run()?;
        info!("latest run found: {:?}", &fc_run);

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            Self::render_cloud_precip_forecast(step_filter, &fc_run)?;
            info!("finished rendering cloud & precipitation forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Wind10m.get_name()) {
            info!("rendering wind 10m forecast...");
            Self::render_wind10m_forecast(step_filter, &fc_run)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            Self::render_temp2m_forecast(step_filter, &fc_run)?;
            info!("finished rendering temperature 2m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
            let vertical_levels = IconD2ModelConfig::get_vertical_level_range();
            let hhl_grids = DwdIconHhlReader::read_hhl_grids(&fc_run, &vertical_levels)?;

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) {
                info!("rendering vertical cloud forecast...");
                Self::render_vertical_clouds_forecast(step_filter, &vertical_levels, &hhl_grids, &fc_run)?;
                info!("finished rendering vertical cloud forecast");
            }

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
                info!("rendering vertical wind forecast...");
                Self::render_vertical_wind_forecast(step_filter, &vertical_levels, &hhl_grids, &fc_run)?;
                info!("finished rendering vertical cloud forecast");
            }
        }

        Ok(())
    }


    fn render_cloud_precip_forecast(
        step_filter: &[usize],
        fc_run: &MeteoForecastRun,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_diff_steps()?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            let cloud_precip_layer = DwdIconCloudPrecipReader::read_layer(fc_run, fc_step)?;
            let weather_layer = DwdIconWeatherReader::read_layer(fc_run, fc_step)?;

            Ok((cloud_precip_layer, weather_layer))
        };

        CloudPrecipForecastRenderer::render(&fc_run, &fc_steps, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_wind10m_forecast(
        step_filter: &[usize],
        fc_run: &MeteoForecastRun,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps()?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            DwdIconWind10mReader::read_layer(fc_run, fc_step)
        };

        Wind10mForecastRenderer::render(&fc_run, &fc_steps, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_temp2m_forecast(
        step_filter: &[usize],
        fc_run: &MeteoForecastRun,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps()?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            DwdIconT2mReader::read_layer(fc_run, fc_step)
        };

        Temp2mForecastRenderer::render(&fc_run, &fc_steps, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_vertical_clouds_forecast(
        step_filter: &[usize],
        vertical_levels: &RangeInclusive<u8>,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        fc_run: &MeteoForecastRun,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps()?;
        let read_fn = |fc_step: &MeteoForecastRunStep| {
            let clc_grids = DwdIconClcReader::read_clc_grids(fc_run, fc_step, vertical_levels)?;
            let layer = MeteoVerticalCloudLayer::new(hhl_grids.clone(), clc_grids);

            Ok(layer)
        };

        VerticalCloudsForecastRenderer::render(
            fc_run,
            &fc_steps,
            step_filter,
            read_fn,
        )?;

        Ok(())
    }


    fn render_vertical_wind_forecast(
        step_filter: &[usize],
        vertical_levels: &RangeInclusive<u8>,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        fc_run: &MeteoForecastRun,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps()?;
        let read_fn = |u_step: &MeteoForecastRunStep| {
            let u_grids = DwdIconUReader::read_u_grids(fc_run, u_step, vertical_levels)?;
            let v_grids = DwdIconVReader::read_v_grids(fc_run, u_step, vertical_levels)?;
            let layer = MeteoVerticalWindLayer::new(hhl_grids.clone(), u_grids, v_grids);

            Ok(layer)
        };

        VerticalWindForecastRenderer::render(
            fc_run,
            &fc_steps,
            step_filter,
            read_fn,
        )?;

        Ok(())
    }


    fn get_forecast_steps() -> Result<Vec<MeteoForecastRunStep>, ForecastRendererError> {
        let steps = MeteoForecastModel::IconD2
            .get_step_range()
            .into_iter()
            .map(|step_nr| MeteoForecastRunStep::new(step_nr, String::new()))
            .collect();

        Ok(steps)
    }


    fn get_forecast_diff_steps() -> Result<Vec<MeteoForecastRunStep>, ForecastRendererError> {
        let steps = MeteoForecastModel::IconD2
            .get_diff_step_range()
            .into_iter()
            .map(|step_nr| MeteoForecastRunStep::new(step_nr, String::new()))
            .collect();

        Ok(steps)
    }
}
