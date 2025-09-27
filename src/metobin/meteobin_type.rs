#[derive(Debug, PartialEq, Clone)]
pub enum MeteobinType {
    Precip,
    Temp2m,
    VerticalClouds,
    VerticalWind,
    Wind10m,
    Weather,
}


impl MeteobinType {
    pub fn get_output_file(&self) -> String {
        match self {
            MeteobinType::Precip => "PRECIP.meteobin".to_string(),
            MeteobinType::Temp2m => "TEMP.meteobin".to_string(),
            MeteobinType::VerticalClouds => "VERTICAL_CLOUDS.meteobin".to_string(),
            MeteobinType::VerticalWind => "VERTICAL_WIND.meteobin".to_string(),
            MeteobinType::Wind10m => "WIND.meteobin".to_string(),
            MeteobinType::Weather => "WW.meteobin".to_string(),
        }
    }
}