use std::ops::RangeInclusive;


#[derive(Debug, PartialEq, Clone)]
pub enum MeteoForecastModel {
    IconGlobal,
    IconEu,
    IconD2,
    IconCh1,
}


impl MeteoForecastModel {
    pub fn get_name(&self) -> &str {
        match self {
            MeteoForecastModel::IconGlobal => "icon-global",
            MeteoForecastModel::IconEu => "icon-eu",
            MeteoForecastModel::IconD2 => "icon-d2",
            MeteoForecastModel::IconCh1 => "icon-ch1",
        }
    }


    pub fn get_step_range(&self) -> RangeInclusive<usize> {
        match self {
            MeteoForecastModel::IconGlobal => 2..=78, // TODO: check and adjust
            MeteoForecastModel::IconEu => 2..=78, // TODO: check and adjust
            MeteoForecastModel::IconD2 => 2..=48,
            MeteoForecastModel::IconCh1 => 2..=33,
        }
    }


    pub fn get_zoom_range(&self) -> (u32, u32) {
        match self {
            MeteoForecastModel::IconGlobal => (0, 5), // TODO: check and adjust
            MeteoForecastModel::IconEu => (0, 6), // TODO: check and adjust
            MeteoForecastModel::IconD2 => (0, 7),
            MeteoForecastModel::IconCh1 => (0, 8),
        }
    }


    pub fn get_vertical_level_range(&self) -> RangeInclusive<u8> {
        match self {
            MeteoForecastModel::IconGlobal => 25..=90, // TODO: check and adjust
            MeteoForecastModel::IconEu => 25..=60, // TODO: check and adjust
            MeteoForecastModel::IconD2 => 25..=65,
            MeteoForecastModel::IconCh1 => 31..=79,
        }
    }
}
