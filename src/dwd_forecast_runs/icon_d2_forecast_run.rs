use chrono::NaiveDate;
use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

pub struct IconD2ForecastRun {
    pub date: NaiveDate,
    pub run_name: IconD2ForecastRunName,
}


impl IconD2ForecastRun {
    pub fn new(
        date: NaiveDate,
        run_name: IconD2ForecastRunName,
    ) -> IconD2ForecastRun {
        return IconD2ForecastRun { date, run_name };
    }
}
