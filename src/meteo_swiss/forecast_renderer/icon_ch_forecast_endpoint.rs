const ICON_CH_ENDPOINT_URL: &str = "https://data.geo.admin.ch/api/stac/v1/search";

pub struct IconChForecastEndpoint;


impl IconChForecastEndpoint {
    pub fn get_endpoint_url() -> &'static str {
        ICON_CH_ENDPOINT_URL
    }
}