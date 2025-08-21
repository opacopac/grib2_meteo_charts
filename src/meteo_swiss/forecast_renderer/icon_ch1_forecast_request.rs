use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub features: Vec<ForecastResponseFeature>,
}


#[derive(Debug, Deserialize)]
pub struct ForecastResponseFeature {
    pub id: String,
    pub properties: ForecastResponseProperties,
    pub assets: ForecastResponseAssets,
}


#[derive(Debug, Deserialize)]
pub struct ForecastResponseProperties {
    pub datetime: String,
    pub title: String,
    #[serde(rename = "forecast:reference_datetime")]
    pub forecast_reference_datetime: String,
    #[serde(rename = "forecast:horizon")]
    pub forecast_horizon: String,
    #[serde(rename = "forecast:variable")]
    pub forecast_variable: String,
}


#[derive(Debug, Deserialize)]
pub struct ForecastResponseAssets {
    #[serde(flatten)]
    pub data: HashMap<String, ForecastResponseAssetProperties>,
}


#[derive(Debug, Deserialize)]
pub struct ForecastResponseAssetProperties {
    pub title: String,
    pub href: String,
}
