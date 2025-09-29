use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use chrono::NaiveDate;


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
        DwdForecastRun { model, start_date, run_name }
    }
}


impl MeteoForecastRun for DwdForecastRun {
    fn get_model_name(&self) -> String {
        self.model.get_name()
    }


    fn get_start_date(&self) -> NaiveDate {
        self.start_date
    }


    fn get_name(&self) -> String {
        self.run_name.get_name()
    }
}
