use std::ops::Add;

use chrono;
use chrono::{Duration, Utc};

use crate::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd_forecast_runs::icon_d2_forecast_run::IconD2ForecastRun;
use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;
use crate::dwd_forecast_runs::icon_d2_forecast_step::IconD2ForecastStep;
use crate::grib2::common::grib2_error::Grib2Error;

pub struct IconD2ForecastRunFinder;


impl IconD2ForecastRunFinder {
    pub fn find_latest_forecast_run() -> Result<IconD2ForecastRun, Grib2Error> {
        let date_today = Utc::today().naive_utc();

        for run in IconD2ForecastRunName::get_all_reversed() {
            let forecast_step = IconD2ForecastStep::new(date_today, run, 0);
            let probe_file_name = IconD2FileClctMod::get_file_url(&forecast_step);
            let response_result = ureq::head(&probe_file_name).call();

            if let Ok(res) = response_result {
                if res.status() == 200 {
                    return Ok(forecast_step.run);
                }
            }
        }

        // TODO: check if yesterday's files really exist
        let date_yesterday = Utc::today().add(Duration::days(-1)).naive_utc();
        let fc_run = IconD2ForecastRun::new(date_yesterday, IconD2ForecastRunName::Run21);

        return Ok(fc_run);
    }
}
