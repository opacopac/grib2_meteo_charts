#[derive(Debug, PartialEq, Clone)]
pub enum MeteoLayerType {
    CloudPrecip,
    Wind10m,
    Temp2m,
    VerticalCloud,
    VerticalWind,
}


impl MeteoLayerType {
    pub fn get_name(&self) -> String {
        match self {
            MeteoLayerType::CloudPrecip => "cloud_precip".to_string(),
            MeteoLayerType::Wind10m => "wind_10m".to_string(),
            MeteoLayerType::Temp2m => "temp_2m".to_string(),
            MeteoLayerType::VerticalCloud => "vertical_cloud".to_string(),
            MeteoLayerType::VerticalWind => "vertical_wind".to_string(),
        }
    }


    pub fn get_output_subdir(&self) -> String {
        match self {
            MeteoLayerType::CloudPrecip => "clct_precip".to_string(),
            MeteoLayerType::Wind10m => "wind".to_string(),
            MeteoLayerType::Temp2m => "temp".to_string(),
            MeteoLayerType::VerticalCloud => "vertical_clouds".to_string(),
            MeteoLayerType::VerticalWind => "vertical_wind".to_string(),
        }
    }
}