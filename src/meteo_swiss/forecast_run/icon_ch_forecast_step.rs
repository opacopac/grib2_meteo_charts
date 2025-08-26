use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;


pub struct IconCh1ForecastStep {
    pub title: String,
    pub horizon: IconChForecastHorizon,
    pub href: String,
}


impl IconCh1ForecastStep {
    pub fn new(title: String, horizon: IconChForecastHorizon, href: String) -> Self {
        Self { title, horizon, href }
    }
}
