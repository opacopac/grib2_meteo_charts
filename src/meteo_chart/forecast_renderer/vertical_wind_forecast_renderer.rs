use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_vertical_wind_layer::MeteoVerticalWindLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::metobin::vertical_wind_metobin::VerticalWindMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};


pub struct VerticalWindForecastRenderer;


const MAX_PARALLEL_STEPS: usize = 3;


impl VerticalWindForecastRenderer {
    pub fn render<S>(
        fc_run: &MeteoForecastRun2,
        fc_steps: &Vec<MeteoForecastRun2Step>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun2Step) -> Result<MeteoVerticalWindLayer, MeteoChartError> + Sync,
    {
        fc_steps
            .par_iter()
            .with_max_len(MAX_PARALLEL_STEPS)
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                info!("creating vertical wind charts, time step {}", fc_step.get_step_nr());
                let layer = read_layer_fn(&fc_step)?;

                // meteobin
                let _ = VerticalWindMeteobin::create_meteobin_file2(&layer, fc_run, fc_step.get_step_nr());

                Ok(())
            })
    }
}
