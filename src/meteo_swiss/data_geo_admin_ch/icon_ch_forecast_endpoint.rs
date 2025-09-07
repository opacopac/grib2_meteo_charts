const DATA_GEO_ADMIN_SEARCH_ENDPOINT_URL: &str = "https://data.geo.admin.ch/api/stac/v1/search";
const ICON_CH1_ASSETS_ENDPOINT_URL: &str = "https://data.geo.admin.ch/api/stac/v1/collections/ch.meteoschweiz.ogd-forecasting-icon-ch1/assets";


pub struct IconChForecastEndpoint;


impl IconChForecastEndpoint {
    pub fn get_search_endpoint_url() -> &'static str {
        DATA_GEO_ADMIN_SEARCH_ENDPOINT_URL
    }

    pub fn get_assets_endpoint_url() -> &'static str { ICON_CH1_ASSETS_ENDPOINT_URL }
}
