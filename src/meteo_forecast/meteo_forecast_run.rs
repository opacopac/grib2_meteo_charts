use crate::meteo_forecast::meteo_forecast_step::MeteoForecastStep;
use chrono::NaiveDate;
use std::ops::RangeInclusive;


pub trait MeteoForecastRunTrait {
    fn get_model_name(&self) -> &String;

    fn get_start_date(&self) -> &NaiveDate;

    fn get_run_name(&self) -> &String;

    fn get_steps(&self) -> &Vec<MeteoForecastStep>;

    fn get_step_range(&self) -> RangeInclusive<usize>;
}


#[derive(Debug)]
pub struct MeteoForecastRun {
    pub model_name: String,
    pub start_date: NaiveDate,
    pub run_name: String,
    pub steps: Vec<MeteoForecastStep>,
}


impl MeteoForecastRun {
    const MIN_STEP: usize = 2;


    fn new(
        model_name: String,
        start_date: NaiveDate,
        run_name: String,
        steps: Vec<MeteoForecastStep>,
    ) -> MeteoForecastRun {
        MeteoForecastRun { model_name, start_date, run_name, steps }
    }
}


impl MeteoForecastRunTrait for MeteoForecastRun {
    fn get_model_name(&self) -> &String {
        &self.model_name
    }


    fn get_start_date(&self) -> &NaiveDate {
        &self.start_date
    }


    fn get_run_name(&self) -> &String {
        &self.run_name
    }


    fn get_steps(&self) -> &Vec<MeteoForecastStep> {
        &self.steps
    }


    fn get_step_range(&self) -> RangeInclusive<usize> {
        Self::MIN_STEP..=self.get_steps().len()
    }
}
