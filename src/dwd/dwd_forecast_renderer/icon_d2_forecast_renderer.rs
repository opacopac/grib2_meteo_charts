use crate::dwd::common::icon_d2_model_config::IconD2ModelConfig;
use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
use crate::dwd::dwd_file_reader::icon_d2_cloud_precip_reader::IconD2CloudPrecipReader;
use crate::dwd::dwd_file_reader::icon_d2_hhl_reader::IconD2HhlReader;
use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_v_reader::IconD2VReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::dwd::dwd_file_reader::icon_d2_weather_reader::IconD2WeatherReader;
use crate::dwd::dwd_file_reader::icon_d2_wind_10m_reader::IconD2Wind10mReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
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
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use log::info;
use std::ops::RangeInclusive;


pub struct IconD2ForecastRenderer;


impl IconD2ForecastRenderer {
    pub fn render_latest_forecasts(
        variable_filter: &Vec<String>,
        step_filter: &Vec<usize>,
    ) -> Result<(), ForecastRendererError> {
        info!("creating latest dwd forecasts...");

        info!("search available forecasts...");
        let latest_run = IconD2ForecastRunFinder::find_latest_forecast_run()?;
        info!("latest run found: {:?}", &latest_run);

        let fc_run = MeteoForecastRun2::new(
            MeteoForecastModel::IconD2,
            latest_run.start_date,
            latest_run.run_name.get_name(),
        );

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            Self::render_cloud_precip_forecast(&step_filter, &latest_run, &fc_run)?;
            info!("finished rendering cloud & precipitation forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Wind10m.get_name()) {
            info!("rendering wind 10m forecast...");
            Self::render_wind10m_forecast(&step_filter, &latest_run, &fc_run)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            Self::render_temp2m_forecast(&step_filter, &latest_run, &fc_run)?;
            info!("finished rendering temperature 2m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
            let vertical_levels = IconD2ModelConfig::get_vertical_level_range();
            let hhl_grids = IconD2HhlReader::read_hhl_grids2(&fc_run, &vertical_levels)?;

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) {
                info!("rendering vertical cloud forecast...");
                Self::render_vertical_clouds_forecast(&step_filter, &vertical_levels, &hhl_grids, &fc_run)?;
                info!("finished rendering vertical cloud forecast");
            }

            if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
                info!("rendering vertical wind forecast...");
                Self::render_vertical_wind_forecast(&step_filter, &vertical_levels, &hhl_grids, &fc_run)?;
                info!("finished rendering vertical cloud forecast");
            }
        }

        Ok(())
    }


    fn render_cloud_precip_forecast(
        step_filter: &Vec<usize>,
        latest_run: &DwdForecastRun,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps_clct = Self::get_forecast_steps(&latest_run, IconD2TotPrecReader::get_file_url)?;
        let fc_steps_prec = Self::get_forecast_steps(&latest_run, IconD2TotPrecReader::get_file_url)?;
        let fc_steps_ceiling = Self::get_forecast_steps(&latest_run, IconD2CeilingReader::get_file_url)?;
        let fc_steps_ww = Self::get_forecast_steps(&latest_run, IconD2WwReader::get_file_url)?;

        let read_fn = |clct_step: &MeteoForecastRun2Step| {
            let step_nr = clct_step.get_step_nr();
            let precip_step0 = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_prec, step_nr - 1)?;
            let precip_step1 = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_prec, step_nr)?;
            let ceiling_step = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_ceiling, step_nr)?;
            let ww_step = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_ww, step_nr)?;

            let cloud_precip_layer = IconD2CloudPrecipReader::read_layer_from_files(
                fc_run,
                clct_step,
                precip_step0,
                precip_step1,
            )?;
            let weather_layer = IconD2WeatherReader::read_layer_from_files(
                fc_run,
                clct_step,
                ceiling_step,
                ww_step,
            )?;

            Ok((cloud_precip_layer, weather_layer))
        };

        CloudPrecipForecastRenderer::render(&fc_run, &fc_steps_clct, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_wind10m_forecast(
        step_filter: &Vec<usize>,
        latest_run: &DwdForecastRun,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps_u10m = Self::get_forecast_steps(latest_run, IconD2U10mReader::get_file_url)?;
        let fc_steps_v10m = Self::get_forecast_steps(latest_run, IconD2V10mReader::get_file_url)?;
        let fc_steps_vmax10m = Self::get_forecast_steps(latest_run, IconD2Vmax10mReader::get_file_url)?;
        let read_fn = |u10m_step: &MeteoForecastRun2Step| {
            let step_nr = u10m_step.get_step_nr();
            let v10m_step = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_v10m, step_nr)?;
            let vmax10m_step = MeteoForecastRun2Step::get_step_by_nr(&fc_steps_vmax10m, step_nr)?;

            IconD2Wind10mReader::read_layer_from_files(
                fc_run,
                u10m_step,
                v10m_step,
                vmax10m_step,
            )
        };

        Wind10mForecastRenderer::render(&fc_run, &fc_steps_u10m, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_temp2m_forecast(
        step_filter: &Vec<usize>,
        latest_run: &DwdForecastRun,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps(&latest_run, IconD2T2mReader::get_file_url)?;
        let read_fn = |step: &MeteoForecastRun2Step| {
            IconD2T2mReader::read_layer_from_file(fc_run, step)
        };

        Temp2mForecastRenderer::render(&fc_run, &fc_steps, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_vertical_clouds_forecast(
        step_filter: &Vec<usize>,
        vertical_levels: &RangeInclusive<u8>,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps_without_url()?;
        let read_fn = |fc_step: &MeteoForecastRun2Step| {
            let clc_grids = IconD2ClcReader::read_clc_grids2(fc_run, fc_step, vertical_levels)?;
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
        step_filter: &Vec<usize>,
        vertical_levels: &RangeInclusive<u8>,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps_without_url()?;
        let read_fn = |u_step: &MeteoForecastRun2Step| {
            let u_grids = IconD2UReader::read_u_grids(fc_run, u_step, vertical_levels)?;
            let v_grids = IconD2VReader::read_v_grids(fc_run, u_step, vertical_levels)?;
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


    fn get_forecast_steps(
        dwd_run: &DwdForecastRun,
        fn_get_url: fn(&DwdForecastStep) -> String,
    ) -> Result<Vec<MeteoForecastRun2Step>, ForecastRendererError> {
        let steps = MeteoForecastModel::IconD2
            .get_step_range()
            .into_iter()
            .map(|step_nr| {
                let dwd_step = DwdForecastStep::new_from_run(dwd_run, step_nr);
                let file_url = fn_get_url(&dwd_step);
                MeteoForecastRun2Step::new(step_nr, file_url)
            })
            .collect();

        Ok(steps)
    }


    fn get_forecast_steps_without_url() -> Result<Vec<MeteoForecastRun2Step>, ForecastRendererError> {
        let steps = MeteoForecastModel::IconD2
            .get_step_range()
            .into_iter()
            .map(|step_nr| MeteoForecastRun2Step::new(step_nr, String::new()))
            .collect();

        Ok(steps)
    }
}
