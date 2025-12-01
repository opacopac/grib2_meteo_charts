use crate::dwd::dwd_forecast_renderer::forecast_renderer_error::ForecastRendererError;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use std::fmt::Display;
use std::ops::RangeInclusive;


#[derive(Debug, PartialEq, Clone)]
pub enum MeteoForecastModel {
    IconGlobal,
    IconEu,
    IconD2,
    IconCh1,
}


impl Display for MeteoForecastModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
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


    pub fn get_forecast_steps(&self) -> Result<Vec<MeteoForecastRunStep>, ForecastRendererError> {
        let steps = self
            .get_step_range()
            .into_iter()
            .map(|step_nr| MeteoForecastRunStep::new(step_nr, String::new()))
            .collect();

        Ok(steps)
    }


    pub fn get_step_range(&self) -> RangeInclusive<usize> {
        match self {
            MeteoForecastModel::IconGlobal => 2..=78, // TODO: check and adjust
            MeteoForecastModel::IconEu => 2..=78, // steps above 78 are currently skipped (3h intervals)
            MeteoForecastModel::IconD2 => 2..=48,
            MeteoForecastModel::IconCh1 => 2..=33,
        }
    }


    pub fn get_forecast_diff_steps(&self) -> Result<Vec<MeteoForecastRunStep>, ForecastRendererError> {
        let steps = self
            .get_diff_step_range()
            .into_iter()
            .map(|step_nr| MeteoForecastRunStep::new(step_nr, String::new()))
            .collect();

        Ok(steps)
    }


    pub fn get_diff_step_range(&self) -> RangeInclusive<usize> {
        let std_range = Self::get_step_range(&self);

        // start at 1 instead of 2 because diff between steps
        (std_range.start() - 1)..=(std_range.end() - 0)
    }


    pub fn get_zoom_range(&self) -> (u32, u32) {
        match self {
            MeteoForecastModel::IconGlobal => (0, 5), // TODO: check and adjust
            MeteoForecastModel::IconEu => (0, 6), // TODO: check and adjust
            MeteoForecastModel::IconD2 => (0, 7),
            MeteoForecastModel::IconCh1 => (0, 8),
        }
    }


    // remarks:
    //  - higher levels have lower numbers
    //  - restrict vertical levels to about 20000ft for better performance
    pub fn get_vertical_level_range(&self) -> RangeInclusive<u8> {
        match self {
            MeteoForecastModel::IconGlobal => 25..=90, // TODO: check and adjust
            MeteoForecastModel::IconEu => 25..=74, // TODO: check and adjust
            MeteoForecastModel::IconD2 => 25..=65,
            MeteoForecastModel::IconCh1 => 31..=79,
        }
    }
}
