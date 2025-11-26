use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::forecast_renderer::wind_10m_chart_renderer::Wind10mChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_wind_10m_layer::MeteoWind10mLayer;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::metobin::wind_metobin::WindMeteobin;
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


pub struct Wind10mForecastRenderer;


impl Wind10mForecastRenderer {
    pub fn render<S>(
        fc_run: &MeteoForecastRun,
        fc_steps: &Vec<MeteoForecastRunStep>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRunStep) -> Result<MeteoWind10mLayer, MeteoChartError> + Sync,
    {
        fc_steps
            .par_iter()
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                info!("creating wind 10m charts, time step {}", fc_step.get_step_nr());
                let layer = read_layer_fn(&fc_step)?;

                // map tiles
                let _ = Wind10mChartRenderer::render_map_tiles(&layer, fc_run, fc_step.get_step_nr());

                // meteobin
                let _ = WindMeteobin::create_meteobin_file(&layer, fc_run, fc_step.get_step_nr());

                Ok(())
            })
    }


    // TODO: new
    pub fn render_all_steps<S>(
        fc_run: &MeteoForecastRun,
        fc_steps: &Vec<MeteoForecastRunStep>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun, &MeteoForecastRunStep) -> Result<MeteoWind10mLayer, MeteoChartError> + Sync + Send,
    {
        fc_steps
            .par_iter()
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                Self::render_single_step(fc_run, fc_step, &read_layer_fn)
            })
    }


    pub fn render_single_step<S>(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun, &MeteoForecastRunStep) -> Result<MeteoWind10mLayer, MeteoChartError> + Sync,
    {
        info!("creating wind 10m charts, time step {}", fc_step.get_step_nr());
        let layer = read_layer_fn(fc_run, fc_step)?;

        // map tiles
        let _ = Wind10mChartRenderer::render_map_tiles(&layer, fc_run, fc_step.get_step_nr());

        // meteobin
        let _ = WindMeteobin::create_meteobin_file(&layer, fc_run, fc_step.get_step_nr());

        Ok(())
    }
}
