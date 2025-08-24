use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_request::{ForecastResponse, ForecastResponseAssets};
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run::IconChForecastRun;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_step::IconCh1ForecastStep;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;

pub const ICON_CH1_POST_URL: &str = "https://data.geo.admin.ch/api/stac/v1/search";
pub const ICON_CH1_WEATHER_MODEL_NAME: &str = "ch.meteoschweiz.ogd-forecasting-icon-ch1";
pub const VARIABLES: [&str; 1] = [
    "T_2M",
];


pub struct IconCh1ForecastUrlsFinder;


impl IconCh1ForecastUrlsFinder {
    pub fn find_latest_forecast_run() -> Result<IconChForecastRun, MeteoSwissError> {
        let body = serde_json::json!({
            "collections": [ICON_CH1_WEATHER_MODEL_NAME],
            "forecast:horizon": "P0DT00H00M00S",
            "forecast:variable": "T_2M", // TODO
            "forecast:perturbed": false
        });
        let response = ureq::post(ICON_CH1_POST_URL)
            .send_json(body)?
            .body_mut()
            .read_json::<ForecastResponse>()?;

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

        let latest_run = IconChForecastRun::new(
            latest_run_naivedate,
            latest_run
        );

        Ok(latest_run)
    }

    
    pub fn find_forecast_file_urls() -> Result<Vec<IconCh1ForecastStep>, MeteoSwissError> {
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
        let result = IconCh1ForecastUrlsFinder::find_forecast_file_urls();
        assert!(result.is_ok(), "Failed to find latest forecast run: {:?}", result.err());
    }
}
