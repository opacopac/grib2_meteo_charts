use std::ops::RangeInclusive;

use chrono::NaiveDate;

use crate::dwd_forecast_runs::icon_d2_forecast_run::IconD2ForecastRun;
use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

pub struct IconD2ForecastStep {
    pub run: IconD2ForecastRun,
    pub step: usize
}


impl IconD2ForecastStep {
    pub fn new(
        date: NaiveDate,
        run_name: IconD2ForecastRunName,
        step: usize
    ) -> IconD2ForecastStep {
        let run = IconD2ForecastRun::new(date, run_name);

        return IconD2ForecastStep{ run, step }
    }


    pub fn new_from_run(
        run: &IconD2ForecastRun,
        step: usize
    ) -> IconD2ForecastStep {
        return IconD2ForecastStep::new(run.date.clone(), run.run_name.clone(), step);
    }


    pub fn get_step_range() -> RangeInclusive<usize> {
        return 0..=48;
    }
}
