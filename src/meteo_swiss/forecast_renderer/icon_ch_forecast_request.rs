use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use serde::Serialize;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;

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


pub struct IconChForecastRequestBuilder {
    pub model: Option<IconChForecastModel>,
    pub forecast_reference_datetime: Option<String>,
    pub forecast_horizon: Option<String>,
    pub forecast_variable: Option<String>,
    pub forecast_perturbed: bool,
}


impl IconChForecastRequestBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            forecast_reference_datetime: None,
            forecast_horizon: None,
            forecast_variable: None,
            forecast_perturbed: false,
        }
    }


    pub fn with_model(mut self, model: IconChForecastModel) -> Self {
        self.model = Some(model);
        self
    }


    pub fn with_forecast_reference_datetime(mut self, datetime: String) -> Self {
        self.forecast_reference_datetime = Some(datetime);
        self
    }


    pub fn with_forecast_horizon(mut self, horizon: String) -> Self {
        self.forecast_horizon = Some(horizon);
        self
    }


    pub fn with_forecast_variable(mut self, variable: IconChForecastVariable) -> Self {
        self.forecast_variable = Some(variable.get_name());
        self
    }


    pub fn with_forecast_perturbed(mut self, perturbed: bool) -> Self {
        self.forecast_perturbed = perturbed;
        self
    }


    pub fn build(self) -> Result<IconChForecastRequest, MeteoSwissError> {
        if self.model.is_none() {
            return Err(MeteoSwissError::InvalidRequestParameters("model is missing".to_string()));
        }

        Ok(IconChForecastRequest {
            collections: vec![self.model.unwrap().get_name().to_string()],
            forecast_reference_datetime: self.forecast_reference_datetime,
            forecast_horizon: self.forecast_horizon,
            forecast_variable: self.forecast_variable,
            forecast_perturbed: self.forecast_perturbed,
        })
    }
}
