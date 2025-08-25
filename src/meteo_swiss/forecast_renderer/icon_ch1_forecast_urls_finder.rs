use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_endpoint::IconChForecastEndpoint;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_request::IconChForecastRequest;
use crate::meteo_swiss::forecast_renderer::icon_ch_forecast_response::{
    ForecastResponseAssets, IconChForecastResponse,
};
use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_step::IconCh1ForecastStep;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_variable::IconChForecastVariable;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;


pub struct IconCh1ForecastUrlsFinder;


impl IconCh1ForecastUrlsFinder {
    pub fn find_forecast_file_urls() -> Result<Vec<IconCh1ForecastStep>, MeteoSwissError> {
        let request = IconChForecastRequest::new(
            vec![IconChForecastModel::IconCh1],
            Some("2025-08-25T12:00:00Z".to_string()), // TODO
            None,
            Some(IconChForecastVariable::T2m), // TODO
            false,
        );
        let body = serde_json::json!(request);
        let response = ureq::post(IconChForecastEndpoint::get_endpoint_url())
            .send_json(body)?
            .body_mut()
            .read_json::<IconChForecastResponse>()?;

        println!("Latest forecast run timestamp: {}", response.timestamp);

        let steps: Vec<IconCh1ForecastStep> = response
            .features
            .iter()
            .map(|f| {
                IconCh1ForecastStep::new(
                    f.properties.title.clone(),
                    f.properties.forecast_horizon.clone(),
                    Self::extract_href_from_assets(&f.assets),
                )
            })
            .collect();

        let sorted_steps = {
            let mut s = steps;
            s.sort_by(|a, b| a.step.cmp(&b.step));
            s
        };

        Ok(sorted_steps)
    }


    fn extract_href_from_assets(assets: &ForecastResponseAssets) -> String {
        for (_key, asset) in &assets.data {
            return asset.href.clone();
        }

        panic!("Could not extract the href from assets");
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_urls_finder::IconCh1ForecastUrlsFinder;

    #[test]
    fn it_finds_the_latest_forecast_urls() {
        let result = IconCh1ForecastUrlsFinder::find_forecast_file_urls();
        assert!(
            result.is_ok(),
            "Failed to find latest forecast run: {:?}",
            result.err()
        );
    }
}
