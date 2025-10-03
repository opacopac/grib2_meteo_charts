use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use chrono::NaiveDate;


pub struct MeteoForecastRun2 {
    model: MeteoForecastModel,
    start_date: NaiveDate,
    run_name: String,
}


impl MeteoForecastRun2 {
    pub fn new(
        model: MeteoForecastModel,
        start_date: NaiveDate,
        run_name: String,
    ) -> MeteoForecastRun2 {
        MeteoForecastRun2 { model, start_date, run_name }
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
