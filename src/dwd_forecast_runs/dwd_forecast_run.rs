use chrono::NaiveDate;
use crate::dwd_forecast_runs::dwd_model_type::DwdModelType;

use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

#[derive(Debug)]
pub struct DwdForecastRun {
    pub model: DwdModelType,
    pub start_date: NaiveDate,
    pub run_name: IconD2ForecastRunName,
}


impl DwdForecastRun {
    pub fn new(
        model: DwdModelType,
        start_date: NaiveDate,
        run_name: IconD2ForecastRunName,
    ) -> DwdForecastRun {
        return DwdForecastRun { model, start_date, run_name };
    }
}
