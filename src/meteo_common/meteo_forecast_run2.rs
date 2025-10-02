use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use chrono::NaiveDate;


pub struct MeteoForecastRun2 {
    model_name: String,
    start_date: NaiveDate,
    run_name: String,
    steps: Vec<MeteoForecastRun2Step>,
    zoom_range: (u32, u32)
}


impl MeteoForecastRun2 {
    pub fn new(
        model_name: String,
        start_date: NaiveDate,
        run_name: String,
        steps: Vec<MeteoForecastRun2Step>,
        zoom_range: (u32, u32)
    ) -> MeteoForecastRun2 {
        MeteoForecastRun2 { model_name, start_date, run_name, steps, zoom_range }
    }


    pub fn get_model_name(&self) -> String {
        self.model_name.clone()
    }


    pub fn get_start_date(&self) -> NaiveDate {
        self.start_date
    }


    pub fn get_name(&self) -> String {
        self.run_name.clone()
    }


    pub fn get_steps(&self) -> &Vec<MeteoForecastRun2Step> {
        &self.steps
    }
    
    
    pub fn get_zoom_range(&self) -> (u32, u32) {
        self.zoom_range
    }
}
