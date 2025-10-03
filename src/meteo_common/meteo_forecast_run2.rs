use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use chrono::NaiveDate;


pub struct MeteoForecastRun2 {
    model: MeteoForecastModel,
    start_date: NaiveDate,
    run_name: String,
    steps: Vec<MeteoForecastRun2Step>,
}


impl MeteoForecastRun2 {
    pub fn new(
        model: MeteoForecastModel,
        start_date: NaiveDate,
        run_name: String,
        steps: Vec<MeteoForecastRun2Step>,
    ) -> MeteoForecastRun2 {
        MeteoForecastRun2 { model, start_date, run_name, steps }
    }


    pub fn get_model(&self) -> &MeteoForecastModel {
        &self.model
    }


    pub fn get_start_date(&self) -> NaiveDate {
        self.start_date
    }


    pub fn get_name(&self) -> &str {
        &self.run_name
    }


    pub fn get_steps(&self) -> &Vec<MeteoForecastRun2Step> {
        &self.steps
    }
}
