use std::fmt::Display;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use chrono::NaiveDate;


#[derive(Debug, PartialEq, Clone)]
pub struct MeteoForecastRun {
    model: MeteoForecastModel,
    start_date: NaiveDate,
    run_name: String,
}


impl Display for MeteoForecastRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "meteo forecast run {{ model: {}, run: {} }}", self.model, self.run_name)
    }
}


impl MeteoForecastRun {
    pub fn new(
        model: MeteoForecastModel,
        start_date: NaiveDate,
        run_name: String,
    ) -> MeteoForecastRun {
        MeteoForecastRun { model, start_date, run_name }
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
}
