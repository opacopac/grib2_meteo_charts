use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;


#[derive(Debug)]
pub struct IconChForecastStep {
    pub title: String,
    pub horizon: IconChForecastHorizon,
    pub href: String,
}


impl IconChForecastStep {
    pub fn new(title: String, horizon: IconChForecastHorizon, href: String) -> Self {
        Self { title, horizon, href }
    }
}
