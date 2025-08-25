use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_request::IconChForecastRequest;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_response::IconChForecastResponse;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_endpoint::IconChForecastEndpoint;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;


pub struct IconCh1ForecastRunFinder;


impl IconCh1ForecastRunFinder {
    pub fn find_latest_forecast_run() -> Result<IconChForecastRun, MeteoSwissError> {
        let request = IconChForecastRequest::new(
            vec![IconChForecastModel::IconCh1],
            None,
            Some("P0DT00H00M00S".to_string()), // TODO
            Some(IconChForecastVariable::T2m), // TODO
            false,
        );
        let body = serde_json::json!(request);
        let response = ureq::post(IconChForecastEndpoint::get_endpoint_url())
            .send_json(body)?
            .body_mut()
            .read_json::<IconChForecastResponse>()?;

        if response.features.is_empty() {
            return Err(MeteoSwissError::NoForecastRunsFound());
        }

        let latest_feature = &response.features[&response.features.len() - 1];
        let datetime_string = &latest_feature.properties.forecast_reference_datetime;
        let datetime = chrono::DateTime::parse_from_rfc3339(datetime_string)
            .map_err(|e| MeteoSwissError::Error(format!("Failed to parse datetime: {}", e)))?;

        let latest_run = IconChForecastRunName::create_from_datetime(&datetime)?;
        let latest_run_naivedate = chrono::DateTime::parse_from_rfc3339(datetime_string)
            .map_err(|e| MeteoSwissError::Error(format!("Failed to parse datetime: {}", e)))?
            .naive_utc()
            .date();

        let latest_run = IconChForecastRun::new(latest_run_naivedate, latest_run);

        Ok(latest_run)
    }
}
