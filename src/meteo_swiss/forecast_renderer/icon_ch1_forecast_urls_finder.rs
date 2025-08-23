use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_request::{ForecastResponse, ForecastResponseAssets};
use crate::meteo_swiss::forecast_run::icon_ch1_forecast_step::IconCh1ForecastStep;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;

pub const ICON_CH1_POST_URL: &str = "https://data.geo.admin.ch/api/stac/v1/search";
pub const ICON_CH1_WEATHER_MODEL_NAME: &str = "ch.meteoschweiz.ogd-forecasting-icon-ch1";
pub const VARIABLES: [&str; 1] = [
    "T_2M",
];


pub struct IconCh1ForecastUrlsFinder;


impl IconCh1ForecastUrlsFinder {
    pub fn find_latest_forecast_run() -> Result<Vec<IconCh1ForecastStep>, MeteoSwissError> {
        let body = serde_json::json!({
            "collections": [ICON_CH1_WEATHER_MODEL_NAME],
            "forecast:reference_datetime": "2025-08-23T18:00:00Z", // TODO
            "forecast:variable": "T_2M", // TODO
            "forecast:perturbed": false
        });
        let response = ureq::post(ICON_CH1_POST_URL)
            .send_json(body)?
            .body_mut()
            .read_json::<ForecastResponse>()?;

        println!("Latest forecast run timestamp: {}", response.timestamp);

        let steps: Vec<IconCh1ForecastStep> = response.features.iter()
            .map(|f| IconCh1ForecastStep::new(
                f.properties.title.clone(),
                f.properties.forecast_horizon.clone(),
                Self::extract_href_from_assets(&f.assets)
            ))
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
        let result = IconCh1ForecastUrlsFinder::find_latest_forecast_run();
        assert!(result.is_ok(), "Failed to find latest forecast run: {:?}", result.err());
    }
}
