use crate::grib2::common::grib2_error::Grib2Error;
use crate::meteo_chart::forecast_renderer::cloud_precip_chart_renderer::CloudPrecipChartRenderer;
use crate::meteo_chart::forecast_renderer::meteo_chart_error::MeteoChartError;
use crate::meteo_chart::meteo_layer::meteo_cloud_precip_layer::MeteoCloudPrecipLayer;
use crate::meteo_chart::meteo_layer::weather_layer::WeatherLayer;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::metobin::precip_metobin::PrecipMeteoBin;
use crate::metobin::weather_metobin::WeatherMeteoBin;
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


pub struct CloudPrecipForecastRenderer;


impl CloudPrecipForecastRenderer {
    pub fn render<S>(
        fc_run: &MeteoForecastRun2,
        fc_steps: &Vec<MeteoForecastRun2Step>,
        step_filter: &Vec<usize>,
        read_layer_fn: S,
    ) -> Result<(), MeteoChartError>
    where
        S: Fn(&MeteoForecastRun2Step) -> Result<(MeteoCloudPrecipLayer, WeatherLayer), Grib2Error> + Sync,
    {
        fc_steps
            .par_iter()
            .try_for_each(|fc_step| {
                if !step_filter.is_empty() && !step_filter.contains(&fc_step.get_step_nr()) {
                    return Ok(());
                }

                info!("creating weather charts, time step {}", fc_step.get_step_nr());
                let (layer_cloud_precip, layer_weather) = read_layer_fn(&fc_step)?;

                // map tiles
                let _ = CloudPrecipChartRenderer::render_map_tiles2(&layer_cloud_precip, fc_run, fc_step.get_step_nr());

                // meteobin
                let _ = PrecipMeteoBin::create_meteobin_file2(&layer_cloud_precip, fc_run, fc_step.get_step_nr());
                let _ = WeatherMeteoBin::create_meteobin_file2(&layer_weather, fc_run, fc_step.get_step_nr());

                Ok(())
            })
    }
}
