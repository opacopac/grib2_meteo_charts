use chrono::NaiveDate;

use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;


#[derive(Debug)]
pub struct IconChForecastRun {
    pub start_date: NaiveDate,
    pub run_name: IconChForecastRunName,
}


impl IconChForecastRun {
    pub fn new(
        start_date: NaiveDate,
        run_name: IconChForecastRunName,
    ) -> IconChForecastRun {
        IconChForecastRun { start_date, run_name }
    }
}
