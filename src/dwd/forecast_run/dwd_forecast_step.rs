use std::ops::RangeInclusive;

use chrono::NaiveDate;

use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;

pub struct DwdForecastStep {
    pub run: DwdForecastRun,
    pub step: usize
}


impl DwdForecastStep {
    pub fn new(
        model: DwdModelType,
        date: NaiveDate,
        run_name: IconD2ForecastRunName,
        step: usize
    ) -> DwdForecastStep {
        let run = DwdForecastRun::new(model, date, run_name);

        return DwdForecastStep { run, step }
    }


    pub fn new_from_run(
        run: &DwdForecastRun,
        step: usize
    ) -> DwdForecastStep {
        return DwdForecastStep::new(run.model.clone(), run.start_date.clone(), run.run_name.clone(), step);
    }


    pub fn get_step_range() -> RangeInclusive<usize> {
        return 2..=48; // TODO
    }
}
