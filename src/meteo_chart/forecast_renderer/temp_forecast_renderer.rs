use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::forecast_renderer::temp_chart_renderer::TempChartRenderer;
use crate::meteo_chart::meteo_layer::meteo_temp_layer::MeteoTempLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::metobin::temp_metobin::TempMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


pub struct TempForecastRenderer;


impl TempForecastRenderer {
    pub fn render<S>(
        fc_run: &MeteoForecastRun2,
        fc_steps: &Vec<MeteoForecastRun2Step>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun2Step) -> Result<MeteoTempLayer, Grib2Error> + Sync,
    {
        fc_steps
            .par_iter()
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                info!("creating temperature charts, time step {}", fc_step.get_step_nr());
                let layer = read_layer_fn(&fc_step)?;

                // map tiles
                let _ = TempChartRenderer::render_map_tiles(&layer, fc_run, fc_step.get_step_nr());

                // meteobin
                let _ = TempMeteoBin::create_meteobin_file(&layer, fc_run, fc_step.get_step_nr());

                Ok(())
            })
    }
}
