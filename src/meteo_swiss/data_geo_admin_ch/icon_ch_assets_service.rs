use crate::meteo_swiss::data_geo_admin_ch::icon_ch_assets_response::{IconChAsset, IconChAssetsResponse};
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_endpoint::IconChForecastEndpoint;
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_request::{IconChForecastRequest, IconChForecastRequestBuilder};
use crate::meteo_swiss::data_geo_admin_ch::icon_ch_forecast_response::{ForecastResponseAssets, ForecastResponseFeature, IconChForecastResponse};
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::debug;

pub struct IconChAssetsService;


impl IconChAssetsService {
    pub fn get() -> Result<IconChAssetsResponse, MeteoSwissError> {
        let url = IconChForecastEndpoint::get_assets_endpoint_url();
        debug!("Request URL: {}", url);

        let response = ureq::get(url)
            .call()?
            .body_mut()
            .read_json::<IconChAssetsResponse>()?;
        debug!("Response: {:?}", response);

        Ok(response)
    }
}


#[cfg(test)]
mod tests {
    use crate::meteo_swiss::data_geo_admin_ch::icon_ch_assets_service::IconChAssetsService;


    #[test]
    fn it_executes_an_assets_query() {
        // given

        // when
        let result = IconChAssetsService::get();

        // then
        assert!(
            result.is_ok(),
            "Failed to find latest forecast run: {:?}",
            result.err()
        );
    }


    #[test]
    fn it_gets_the_hor_and_vertical_constants_assets() {
        // given

        // when
        let result = IconChAssetsService::get().expect("Failed to get assets");
        let hor_consts_opt = result.get_horizontal_constants();
        let vert_consts_opt = result.get_vertical_constants();

        // then
        assert!(hor_consts_opt.is_some());
        assert!(vert_consts_opt.is_some());
        let hor_consts = hor_consts_opt.as_ref().unwrap();
        let vert_consts = vert_consts_opt.as_ref().unwrap();

        assert!(hor_consts.href.starts_with("https://"));
        assert!(vert_consts.href.starts_with("https://"));
        assert!(hor_consts.href.contains(".grib2"));
        assert!(vert_consts.href.contains(".grib2"));
    }
}
