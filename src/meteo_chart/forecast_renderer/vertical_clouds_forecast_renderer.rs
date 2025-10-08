use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::metobin::vertical_cloud_metobin::VerticalCloudMeteobin;
use log::info;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};


pub struct VerticalCloudsForecastRenderer;


const MAX_PARALLEL_STEPS: usize = 3;


impl VerticalCloudsForecastRenderer {
    pub fn render<S>(
        fc_run: &MeteoForecastRun2,
        fc_steps: &Vec<MeteoForecastRun2Step>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun2Step) -> Result<MeteoVerticalCloudLayer, Grib2Error> + Sync,
    {
        fc_steps
            .par_iter()
            .with_max_len(MAX_PARALLEL_STEPS)
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                info!("creating vertical cloud charts, time step {}", fc_step.get_step_nr());
                let layer = read_layer_fn(&fc_step)?;

                // meteobin
                let _ = VerticalCloudMeteobin::create_meteobin_file2(&layer, fc_run, fc_step.get_step_nr());

                Ok(())
            })
    }
}
