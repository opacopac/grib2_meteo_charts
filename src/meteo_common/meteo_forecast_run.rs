use chrono::NaiveDate;


pub trait MeteoForecastRun {
    fn get_model_name(&self) -> String;
    fn get_start_date(&self) -> NaiveDate;
    fn get_name(&self) -> String;
}
