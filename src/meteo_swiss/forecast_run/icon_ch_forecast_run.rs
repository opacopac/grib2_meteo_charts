use crate::meteo_swiss::forecast_run::icon_ch_forecast_model::IconChForecastModel;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_run_name::IconChForecastRunName;
use crate::meteo_swiss::forecast_run::icon_ch_forecast_step::IconChForecastStep;
use chrono::NaiveDate;
use std::ops::RangeInclusive;


#[derive(Debug)]
pub struct IconChForecastRun {
    pub model: IconChForecastModel,
    pub start_date: NaiveDate,
    pub run_name: IconChForecastRunName,
    pub steps: Vec<IconChForecastStep>,
}


impl IconChForecastRun {
    const MIN_STEP: usize = 2; // TODO: why not 1?


    pub fn new(
        model: IconChForecastModel,
        start_date: NaiveDate,
        run_name: IconChForecastRunName,
        steps: Vec<IconChForecastStep>,
    ) -> IconChForecastRun {
        IconChForecastRun { model, start_date, run_name, steps }
    }


    pub fn get_step_range(&self) -> RangeInclusive<usize> {
        Self::MIN_STEP..=self.steps.len() - 1
    }
}
