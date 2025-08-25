#[derive(Debug, PartialEq, Clone)]
pub enum IconChForecastModel {
    IconCh1,
    IconCh2,
}


impl IconChForecastModel {
    pub fn get_name(&self) -> String {
        match self {
            IconChForecastModel::IconCh1 => "ch.meteoschweiz.ogd-forecasting-icon-ch1".to_string(),
            IconChForecastModel::IconCh2 => "ch.meteoschweiz.ogd-forecasting-icon-ch2".to_string(),
        }
    }
}
