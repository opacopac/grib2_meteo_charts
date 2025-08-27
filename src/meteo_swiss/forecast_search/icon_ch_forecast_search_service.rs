use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_step::IconCh1ForecastStep;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::forecast_search::icon_ch_forecast_endpoint::IconChForecastEndpoint;
use crate::meteo_swiss::forecast_search::icon_ch_forecast_request::{IconChForecastRequest, IconChForecastRequestBuilder};
use crate::meteo_swiss::forecast_search::icon_ch_forecast_response::{ForecastResponseAssets, ForecastResponseFeature, IconChForecastResponse};
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::debug;
use serde_json::to_string;

pub struct IconChForecastSearchService;


impl IconChForecastSearchService {
    pub fn search(request: &IconChForecastRequest) -> Result<IconChForecastResponse, MeteoSwissError> {
        let url = IconChForecastEndpoint::get_endpoint_url();
        debug!("Request URL: {}", url);

        let body = serde_json::json!(request);
        debug!("Request Body: {}", to_string(&body).expect("Failed to convert to JSON string"));

        let response = ureq::post(url)
            .send_json(body)?
            .body_mut()
            .read_json::<IconChForecastResponse>()?;
        debug!("Response: {:?}", response);

        Ok(response)
    }


    pub fn find_latest_ref_datetime(
        model: &IconChForecastModel,
    ) -> Result<IconChForecastReferenceDateTime, MeteoSwissError> {
        let variable = IconChForecastVariable::T2m;
        let horizon = IconChForecastHorizon::create_zero();
        let request = IconChForecastRequestBuilder::new()
            .with_model(model)
            .with_forecast_variable(&variable)
            .with_forecast_horizon(&horizon)
            .with_forecast_perturbed(false)
            .build()?;

        let response = Self::search(&request)?;
        if response.features.is_empty() {
            return Err(MeteoSwissError::NoForecastRunsFound());
        }

        let latest_feature = &response.features[&response.features.len() - 1];
        let datetime_string = &latest_feature.properties.forecast_reference_datetime;
        let reference_datetime = IconChForecastReferenceDateTime::from_str(datetime_string)?;

        Ok(reference_datetime)
    }


    pub fn find_forecast_file_urls(
        model: &IconChForecastModel,
        variable: &IconChForecastVariable,
        reference_datetime: &IconChForecastReferenceDateTime,
    ) -> Result<Vec<IconCh1ForecastStep>, MeteoSwissError> {
        let request = IconChForecastRequestBuilder::new()
            .with_model(model)
            .with_forecast_variable(variable)
            .with_forecast_reference_datetime(reference_datetime)
            .with_forecast_perturbed(false)
            .build()?;

        let response = IconChForecastSearchService::search(&request)?;

        let steps: Result<Vec<IconCh1ForecastStep>, MeteoSwissError> = response
            .features
            .iter()
            .map(|f| Self::create_step_from_feature(f))
            .collect();

        let mut steps = steps?;
        steps.sort_by_key(|s| s.horizon.get_step());

        Ok(steps)
    }


    fn create_step_from_feature(feature: &ForecastResponseFeature) -> Result<IconCh1ForecastStep, MeteoSwissError> {
        let title = feature.properties.title.clone();
        let horizon_str = feature.properties.forecast_horizon.clone();
        let horizon = IconChForecastHorizon::from_str(&horizon_str)?;
        let href = Self::extract_href_from_assets(&feature.assets)?;

        Ok(IconCh1ForecastStep::new(title, horizon, href))
    }


    fn extract_href_from_assets(assets: &ForecastResponseAssets) -> Result<String, MeteoSwissError> {
        for (_key, asset) in &assets.data {
            return Ok(asset.href.clone());
        }

        Err(MeteoSwissError::InvalidParameters("No URL found in assets".to_string()))
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_horizon::IconChForecastHorizon;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_reference_datetime::IconChForecastReferenceDateTime;
    use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
    use crate::meteo_swiss::forecast_search::icon_ch_forecast_request::IconChForecastRequestBuilder;
    use crate::meteo_swiss::forecast_search::icon_ch_forecast_search_service::IconChForecastSearchService;

    #[test]
    fn it_executes_a_forecast_search() {
        // given
        let model = IconChForecastModel::IconCh1;
        let variable = IconChForecastVariable::T2m;
        let horizon = IconChForecastHorizon::create_zero();
        let now_minus_3h = chrono::Utc::now() - chrono::Duration::hours(3);
        let reference_datetime = IconChForecastReferenceDateTime::get_latest(now_minus_3h);
        let request = IconChForecastRequestBuilder::new()
            .with_model(&model)
            .with_forecast_variable(&variable)
            .with_forecast_horizon(&horizon)
            .with_forecast_perturbed(false)
            .with_forecast_reference_datetime(&reference_datetime)
            .build()
            .unwrap();

        // when
        let result = IconChForecastSearchService::search(&request);

        // then
        assert!(
            result.is_ok(),
            "Failed to find latest forecast run: {:?}",
            result.err()
        );
    }


    #[test]
    fn it_gets_the_latest_reference_datetime() {
        // given
        let model = IconChForecastModel::IconCh1;

        // when
        let result = IconChForecastSearchService::find_latest_ref_datetime(&model);

        // then
        assert!(result.is_ok());
        let ref_datetime = result.unwrap();
        println!("Latest reference datetime: {}", ref_datetime.datetime);
    }


    #[test]
    fn it_finds_the_latest_forecast_urls() {
        // given
        let model = IconChForecastModel::IconCh1;
        let variable = IconChForecastVariable::T2m;
        let now_minus_3h = chrono::Utc::now() - chrono::Duration::hours(3);
        let reference_datetime = IconChForecastReferenceDateTime::get_latest(now_minus_3h);

        // when
        let result = IconChForecastSearchService::find_forecast_file_urls(
            &model,
            &variable,
            &reference_datetime,
        );

        // then
        assert!(
            result.is_ok(),
            "Failed to find latest forecast run: {:?}",
            result.err()
        );

        let steps = result.unwrap();
        assert!(!steps.is_empty(), "No forecast steps found");
        for step in &steps {
            println!("Step: {}, Horizon: {} days, {} hours, URL: {}", step.title, step.horizon.days, step.horizon.hours, step.href);
        }
    }
}
