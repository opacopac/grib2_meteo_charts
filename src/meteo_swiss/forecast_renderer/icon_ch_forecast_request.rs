use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct IconChForecastRequest {
    pub collections: Vec<String>,

    #[serde(rename = "forecast:reference_datetime")]
    pub forecast_reference_datetime: Option<String>,

    #[serde(rename = "forecast:horizon")]
    pub forecast_horizon: Option<String>,

    #[serde(rename = "forecast:variable")]
    pub forecast_variable: Option<String>,

    #[serde(rename = "forecast:perturbed")]
    pub forecast_perturbed: bool,
}


impl IconChForecastRequest {
    pub fn new(
        collections: Vec<IconChForecastModel>,
        forecast_reference_datetime: Option<String>,
        forecast_horizon: Option<String>,
        forecast_variable: Option<IconChForecastVariable>,
        forecast_perturbed: bool,
    ) -> Self {
        Self {
            collections: collections
                .into_iter()
                .map(|c| c.get_name().to_string())
                .collect(),
            forecast_reference_datetime,
            forecast_horizon,
            forecast_variable: forecast_variable
                .map(|v| v.get_name()),
            forecast_perturbed,
        }
    }
}
