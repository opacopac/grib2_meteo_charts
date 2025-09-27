#[derive(Debug, PartialEq, Clone)]
pub enum MeteoLayer {
    CloudPrecip,
    Wind10m,
    Temp2m,
    VerticalCloud,
    VerticalWind,
}


impl MeteoLayer {
    pub fn get_name(&self) -> String {
        match self {
            MeteoLayer::CloudPrecip => "cloud_precip".to_string(),
            MeteoLayer::Wind10m => "wind_10m".to_string(),
            MeteoLayer::Temp2m => "temp_2m".to_string(),
            MeteoLayer::VerticalCloud => "vertical_cloud".to_string(),
            MeteoLayer::VerticalWind => "vertical_wind".to_string(),
        }
    }


    pub fn get_output_subdir(&self) -> String {
        match self {
            MeteoLayer::CloudPrecip => "clct_precip".to_string(),
            MeteoLayer::Wind10m => "wind".to_string(),
            MeteoLayer::Temp2m => "temp".to_string(),
            MeteoLayer::VerticalCloud => "vertical_clouds".to_string(),
            MeteoLayer::VerticalWind => "vertical_wind".to_string(),
        }
    }
}