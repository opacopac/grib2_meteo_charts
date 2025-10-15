use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::dwd::dwd_file_reader::icon_d2_wind_10m_reader::IconD2Wind10mReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_cloud_precip_forecast_renderer::IconD2CloudPrecipRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_cloud_forecast_renderer::IconD2VerticalCloudForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_wind_forecast_renderer::IconD2VerticalWindForecastRenderer;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::meteo_chart::forecast_renderer::temp_2m_forecast_renderer::Temp2mForecastRenderer;
use crate::meteo_chart::forecast_renderer::wind_10m_forecast_renderer::Wind10mForecastRenderer;
use crate::meteo_chart::meteo_layer::meteo_layer_type::MeteoLayerType;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use log::info;


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

        let fc_run = Self::get_forecast_run(&latest_run)?;

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::CloudPrecip.get_name()) {
            info!("rendering cloud & precipitation forecast...");
            IconD2CloudPrecipRenderer::render(&latest_run, &step_filter)?;
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

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalCloud.get_name()) {
            info!("rendering vertical cloud forecast...");
            IconD2VerticalCloudForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering vertical cloud forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::VerticalWind.get_name()) {
            info!("rendering vertical wind forecast...");
            IconD2VerticalWindForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering vertical cloud forecast");
        }

        Ok(())
    }


    fn get_forecast_run(
        dwd_run: &DwdForecastRun
    ) -> Result<MeteoForecastRun2, ForecastRendererError> {
        let run = MeteoForecastRun2::new(
            MeteoForecastModel::IconD2,
            dwd_run.start_date,
            dwd_run.run_name.get_name(),
        );

        Ok(run)
    }


    fn render_temp2m_forecast(
        step_filter: &&Vec<usize>,
        latest_run: &DwdForecastRun,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps = Self::get_forecast_steps(&latest_run, IconD2T2mReader::get_file_url)?;
        let read_fn = |step: &MeteoForecastRun2Step| {
            IconD2T2mReader::read_layer_from_file(&step)
        };

        Temp2mForecastRenderer::render(&fc_run, &fc_steps, &step_filter, read_fn)?;

        Ok(())
    }


    fn render_wind10m_forecast(
        step_filter: &&Vec<usize>,
        latest_run: &DwdForecastRun,
        fc_run: &MeteoForecastRun2,
    ) -> Result<(), ForecastRendererError> {
        let fc_steps_u10m = Self::get_forecast_steps(latest_run, IconD2U10mReader::get_file_url)?;
        let fc_steps_v10m = Self::get_forecast_steps(latest_run, IconD2V10mReader::get_file_url)?;
        let fc_steps_vmax10m = Self::get_forecast_steps(latest_run, IconD2Vmax10mReader::get_file_url)?;
        let read_fn = |u10m_step: &MeteoForecastRun2Step| {
            let step_idx = u10m_step.get_step_nr();
            let v10m_step = &fc_steps_v10m[step_idx];
            let vmax10m_step = &fc_steps_vmax10m[step_idx];

            IconD2Wind10mReader::read_layer_from_files(
                u10m_step,
                v10m_step,
                vmax10m_step,
            )
        };

        Wind10mForecastRenderer::render(&fc_run, &fc_steps_u10m, &step_filter, read_fn)?;

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
}
