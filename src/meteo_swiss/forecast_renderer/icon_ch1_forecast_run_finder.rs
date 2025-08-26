use log::debug;
use serde_json::to_string;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_endpoint::IconChForecastEndpoint;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_request::IconChForecastRequestBuilder;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_response::IconChForecastResponse;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;


pub struct IconCh1ForecastRunFinder;


impl IconCh1ForecastRunFinder {
    pub fn find_latest_ref_datetime() -> Result<IconChForecastReferenceDateTime, MeteoSwissError> {
        let model = IconChForecastModel::IconCh1;
        let variable = IconChForecastVariable::T2m;
        let horizon = IconChForecastHorizon::create_zero();
        let request = IconChForecastRequestBuilder::new()
            .with_model(model)
            .with_forecast_variable(variable)
            .with_forecast_horizon(horizon)
            .with_forecast_perturbed(false)
            .build()?;

        let url = IconChForecastEndpoint::get_endpoint_url();
        debug!("Request URL: {}", url);
        let body = serde_json::json!(request);
        debug!("Request Body: {}", to_string(&body).expect("Failed to convert to JSON string"));

        let response = ureq::post(url)
            .send_json(body)?
            .body_mut()
            .read_json::<IconChForecastResponse>()?;

        if response.features.is_empty() {
            return Err(MeteoSwissError::NoForecastRunsFound());
        }

        let latest_feature = &response.features[&response.features.len() - 1];
        let datetime_string = &latest_feature.properties.forecast_reference_datetime;
        let reference_datetime = IconChForecastReferenceDateTime::from_str(datetime_string)?;

        Ok(reference_datetime)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_gets_the_latest_reference_datetime() {
        // given

        // when
        let result = super::IconCh1ForecastRunFinder::find_latest_ref_datetime();

        // then
        assert!(result.is_ok());
        let ref_datetime = result.unwrap();
        println!("Latest reference datetime: {}", ref_datetime.datetime);
    }
}