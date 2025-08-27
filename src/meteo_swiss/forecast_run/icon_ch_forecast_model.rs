#[derive(Debug, PartialEq, Clone)]
pub enum IconChForecastModel {
    IconCh1,
    IconCh2,
}


impl IconChForecastModel {
    pub fn get_name(&self) -> &str {
        match self {
            IconChForecastModel::IconCh1 => "icon-ch1",
            IconChForecastModel::IconCh2 => "icon-ch2",
        }
    }

    
    pub fn get_search_request_name(&self) -> &str {
        match self {
            IconChForecastModel::IconCh1 => "ch.meteoschweiz.ogd-forecasting-icon-ch1",
            IconChForecastModel::IconCh2 => "ch.meteoschweiz.ogd-forecasting-icon-ch2",
        }
    }
}
