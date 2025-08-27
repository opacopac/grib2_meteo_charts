use crate::meteo_forecast::meteo_forecast_run::MeteoForecastRun;


pub struct MeteoForecastStep<'a> {
    pub run: &'a MeteoForecastRun,
    pub step: usize,
}


impl MeteoForecastStep<'_> {
    pub fn new(
        run: &MeteoForecastRun,
        step: usize,
    ) -> MeteoForecastStep {
        MeteoForecastStep { run, step }
    }
}
