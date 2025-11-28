use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;

pub struct IconD2FileService;


impl IconD2FileService {
    pub fn new() -> Self {
        IconD2FileService {}
    }

    
    pub fn get_file_url(&self, fc_run: &MeteoForecastRun, fc_step: &MeteoForecastRunStep) -> String {
        todo!("not implemented yet")
    }
}