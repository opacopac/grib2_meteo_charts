use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IconChForecastRequest {
    pub collections: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forecast:reference_datetime")]
    pub forecast_reference_datetime: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forecast:horizon")]
    pub forecast_horizon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
                .map(|c| c.get_search_request_name().to_string())
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


    pub fn with_model(mut self, model: &IconChForecastModel) -> Self {
        self.model = Some(model.clone());
        self
    }


    pub fn with_forecast_reference_datetime(mut self, datetime: &IconChForecastReferenceDateTime) -> Self {
        self.forecast_reference_datetime = Some(datetime.get_name());
        self
    }


    pub fn with_forecast_horizon(mut self, horizon: &IconChForecastHorizon) -> Self {
        self.forecast_horizon = Some(horizon.get_name());
        self
    }


    pub fn with_forecast_variable(mut self, variable: &IconChForecastVariable) -> Self {
        self.forecast_variable = Some(variable.get_name());
        self
    }


    pub fn with_forecast_perturbed(mut self, perturbed: bool) -> Self {
        self.forecast_perturbed = perturbed;
        self
    }


    pub fn build(self) -> Result<IconChForecastRequest, MeteoSwissError> {
        if self.model.is_none() {
            return Err(MeteoSwissError::InvalidParameters("model is missing".to_string()));
        }

        Ok(IconChForecastRequest {
            collections: vec![self.model.unwrap().get_search_request_name().to_string()],
            forecast_reference_datetime: self.forecast_reference_datetime,
            forecast_horizon: self.forecast_horizon,
            forecast_variable: self.forecast_variable,
            forecast_perturbed: self.forecast_perturbed,
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
    use crate::meteo_swiss::forecast_search::icon_ch_forecast_request::IconChForecastRequestBuilder;

    #[test]
    fn it_builds_a_forecast_request() {
        // given
        let model = IconChForecastModel::IconCh1;
        let variable = IconChForecastVariable::T2m;
        let datetime_str = "2025-08-25T12:00:00Z";
        let datetime_reference = IconChForecastReferenceDateTime::from_str(datetime_str).unwrap();
        let horizon = IconChForecastHorizon::new(1, 6);
        let builder = IconChForecastRequestBuilder::new()
            .with_model(&model)
            .with_forecast_variable(&variable)
            .with_forecast_reference_datetime(&datetime_reference)
            .with_forecast_horizon(&horizon)
            .with_forecast_perturbed(false);

        // when
        let request = builder.build();

        // then
        assert!(request.is_ok());

        let request = request.unwrap();
        assert_eq!(request.collections, vec![model.get_search_request_name()]);
        assert_eq!(request.forecast_variable, Some(variable.get_name()));
        assert_eq!(request.forecast_reference_datetime, Some(datetime_str.to_string()));
        assert_eq!(request.forecast_horizon, Some(horizon.get_name()));
        assert_eq!(request.forecast_perturbed, false);
    }


    #[test]
    fn it_fails_to_build_a_forecast_request_without_model() {
        // given
        let horizon = IconChForecastHorizon::create_zero();
        let variable = IconChForecastVariable::T2m;
        let builder = IconChForecastRequestBuilder::new()
            .with_forecast_horizon(&horizon)
            .with_forecast_variable(&variable)
            .with_forecast_perturbed(false);

        // when
        let request = builder.build();

        // then
        assert!(request.is_err());
    }
}
