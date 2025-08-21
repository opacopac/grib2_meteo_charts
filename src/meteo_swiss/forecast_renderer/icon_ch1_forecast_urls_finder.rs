use crate::meteo_swiss::forecast_renderer::icon_ch1_forecast_request::ForecastResponse;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;

pub const ICON_CH1_POST_URL: &str = "https://data.geo.admin.ch/api/stac/v1/search";
pub const ICON_CH1_WEATHER_MODEL_NAME: &str = "ch.meteoschweiz.ogd-forecasting-icon-ch1";
pub const VARIABLES: [&str; 1] = [
    "T_2M",
];


pub struct IconCh1ForecastUrlsFinder;


impl IconCh1ForecastUrlsFinder {
    pub fn find_latest_forecast_run() -> Result<Vec<String>, MeteoSwissError> {
        let body = serde_json::json!({
            "collections": [ICON_CH1_WEATHER_MODEL_NAME],
            "forecast:variable": "T_2M",
            "forecast:perturbed": false
        });
        let response = ureq::post(ICON_CH1_POST_URL)
            .send_json(body)?
            .body_mut()
            .read_json::<ForecastResponse>()?;

        println!("Latest forecast run timestamp: {}", response.timestamp);

        let prop_tile_list = response.features.iter()
            //.filter(|f| f.properties.forecast_variable == VARIABLES[0])
            .map(|f| f.properties.title.clone())
            .collect();

        Ok(prop_tile_list)
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
