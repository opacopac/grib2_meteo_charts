use chrono::{FixedOffset, Timelike};
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;

#[derive(Debug, PartialEq, Clone)]
pub enum IconChForecastRunName {
    Run00,
    Run03,
    Run06,
    Run09,
    Run12,
    Run15,
    Run18,
    Run21
}


impl IconChForecastRunName {
    pub fn create_from_datetime(datetime: &chrono::DateTime<FixedOffset>) -> Result<IconChForecastRunName, MeteoSwissError> {
        let hour = datetime.hour();

        match hour {
            0 => Ok(IconChForecastRunName::Run00),
            3 => Ok(IconChForecastRunName::Run03),
            6 => Ok(IconChForecastRunName::Run06),
            9 => Ok(IconChForecastRunName::Run09),
            12 => Ok(IconChForecastRunName::Run12),
            15 => Ok(IconChForecastRunName::Run15),
            18 => Ok(IconChForecastRunName::Run18),
            21 => Ok(IconChForecastRunName::Run21),
            _ => Err(MeteoSwissError::Error(format!("Invalid hour for forecast run: {}", hour)))
        }
    }
    
    
    pub fn get_all() -> Vec<IconChForecastRunName> {
        return vec![
            IconChForecastRunName::Run00,
            IconChForecastRunName::Run03,
            IconChForecastRunName::Run06,
            IconChForecastRunName::Run09,
            IconChForecastRunName::Run12,
            IconChForecastRunName::Run15,
            IconChForecastRunName::Run18,
            IconChForecastRunName::Run21
        ];
    }


    pub fn get_all_reversed() -> Vec<IconChForecastRunName> {
        let mut all = IconChForecastRunName::get_all();
        all.reverse();

        return all;
    }


    pub fn get_name(&self) -> String {
        return match self {
            IconChForecastRunName::Run00 => "00".to_string(),
            IconChForecastRunName::Run03 => "03".to_string(),
            IconChForecastRunName::Run06 => "06".to_string(),
            IconChForecastRunName::Run09 => "09".to_string(),
            IconChForecastRunName::Run12 => "12".to_string(),
            IconChForecastRunName::Run15 => "15".to_string(),
            IconChForecastRunName::Run18 => "18".to_string(),
            IconChForecastRunName::Run21 => "21".to_string()
        }
    }
}
