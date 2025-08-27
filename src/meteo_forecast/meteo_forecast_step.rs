use chrono::{DateTime, Utc};


#[derive(Debug)]
pub struct MeteoForecastStep {
    pub step: usize,
    pub date_time: DateTime<Utc>,
}


impl MeteoForecastStep {
    pub fn new(
        step: usize,
        date_time: DateTime<Utc>,
    ) -> MeteoForecastStep {
        MeteoForecastStep { step, date_time }
    }
}
