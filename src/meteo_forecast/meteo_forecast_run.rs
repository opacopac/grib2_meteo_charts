use chrono::NaiveDate;


#[derive(Debug)]
pub struct MeteoForecastRun {
    pub model_name: String,
    pub start_date: NaiveDate,
    pub run_name: String,
    pub step_count: usize,
}


impl MeteoForecastRun {
    pub fn new(
        model_name: String,
        start_date: NaiveDate,
        run_name: String,
        step_count: usize,
    ) -> MeteoForecastRun {
        MeteoForecastRun { model_name, start_date, run_name, step_count }
    }


    pub fn get_step_range(&self) -> std::ops::RangeInclusive<usize> {
        2..=self.step_count
    }
}
