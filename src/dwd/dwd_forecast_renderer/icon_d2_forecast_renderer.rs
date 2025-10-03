use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::dwd::dwd_forecast_renderer::icon_d2_cloud_precip_forecast_renderer::IconD2CloudPrecipRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_forecast_run_finder::IconD2ForecastRunFinder;
use crate::dwd::dwd_forecast_renderer::icon_d2_temp_forecast_renderer::IconD2TempForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_cloud_forecast_renderer::IconD2VerticalCloudForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_vertical_wind_forecast_renderer::IconD2VerticalWindForecastRenderer;
use crate::dwd::dwd_forecast_renderer::icon_d2_wind_10m_forecast_renderer::IconD2Wind10mForecastRenderer;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
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
            IconD2Wind10mForecastRenderer::render(&latest_run, &step_filter)?;
            info!("finished rendering wind 10m forecast");
        }

        if variable_filter.is_empty() || variable_filter.contains(&MeteoLayerType::Temp2m.get_name()) {
            info!("rendering temperature 2m forecast...");
            let fc_steps = Self::get_temp_forecast_steps(&latest_run)?;
            //IconD2TempForecastRenderer::render(&latest_run, &step_filter)?;
            IconD2TempForecastRenderer::render2(&fc_run, &fc_steps, &step_filter)?;
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


    fn get_temp_forecast_steps(
        dwd_run: &DwdForecastRun
    ) -> Result<Vec<MeteoForecastRun2Step>, ForecastRendererError> {
        let steps = MeteoForecastModel::IconD2
            .get_step_range()
            .into_iter()
            .map(|step_nr| {
                let dwd_step = DwdForecastStep::new_from_run(dwd_run, step_nr);
                MeteoForecastRun2Step::new(
                    step_nr,
                    IconD2T2mReader::get_file_url(&dwd_step),
                )
            })
            .collect();

        Ok(steps)
    }
}
