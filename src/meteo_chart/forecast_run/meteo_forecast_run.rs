use crate::meteo_chart::forecast_run::meteo_forecast_step::MeteoForecastStep;
use chrono::NaiveDate;
use std::ops::RangeInclusive;


#[derive(Debug)]
pub struct MeteoForecastRun {
    pub model_name: String,
    pub start_date: NaiveDate,
    pub run_name: String,
    pub steps: Vec<MeteoForecastStep>,
}


impl MeteoForecastRun {
    const MIN_STEP: usize = 2;


    pub fn new(
        model_name: String,
        start_date: NaiveDate,
        run_name: String,
        steps: Vec<MeteoForecastStep>,
    ) -> MeteoForecastRun {
        MeteoForecastRun { model_name, start_date, run_name, steps }
    }


    pub fn get_step_range(&self) -> RangeInclusive<usize> {
        Self::MIN_STEP..=self.steps.len()
    }
}
