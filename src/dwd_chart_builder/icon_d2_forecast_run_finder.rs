use std::ops::Add;

use chrono;
use chrono::{Duration, Utc};
use crate::dwd_files::icon_d2_file_ceiling::IconD2FileCeiling;

use crate::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
use crate::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
use crate::dwd_files::icon_d2_file_u_10m::IconD2FileU10m;
use crate::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
use crate::dwd_files::icon_d2_file_vmax_10m::IconD2FileVmax10m;
use crate::dwd_files::icon_d2_file_ww::IconD2FileWw;
use crate::dwd_forecast_runs::icon_d2_forecast_run::IconD2ForecastRun;
use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;
use crate::dwd_forecast_runs::icon_d2_forecast_step::IconD2ForecastStep;
use crate::grib2::common::grib2_error::Grib2Error;

pub struct IconD2ForecastRunFinder;


impl IconD2ForecastRunFinder {
    pub fn find_latest_forecast_run() -> Result<IconD2ForecastRun, Grib2Error> {
        let date_today = Utc::today().naive_utc();

        // return Ok(IconD2ForecastRun::new(date_today, IconD2ForecastRunName::Run12));
        let last_step = IconD2ForecastStep::get_step_range().end().clone();

        // check each run
        for run in IconD2ForecastRunName::get_all_reversed() {
            let forecast_step = IconD2ForecastStep::new(date_today, run, last_step);
            let probe_file_names = Self::get_probe_file_names(&forecast_step);

            // check all probe files
            let mut all_files_found: bool = true;
            for probe_file_name in probe_file_names {
                let response_result = ureq::head(&probe_file_name).call();

                match response_result {
                    Ok(res) => {
                        if res.status() != 200 {
                            all_files_found = false;
                            break;
                        }
                    }
                    _ => {
                        all_files_found = false;
                        break;
                    }
                }
            }

            if all_files_found {
                return Ok(forecast_step.run);
            }
        }

        // TODO: check if yesterday's files really exist
        let date_yesterday = Utc::today().add(Duration::days(-1)).naive_utc();
        let fc_run = IconD2ForecastRun::new(date_yesterday, IconD2ForecastRunName::Run21);

        return Ok(fc_run);
    }


    pub fn get_probe_file_names(step: &IconD2ForecastStep) -> Vec<String> {
        return vec![
            IconD2FileClctMod::get_file_url(step),
            IconD2FileTotPrec::get_file_url(step),
            IconD2FileCeiling::get_file_url(step),
            IconD2FileWw::get_file_url(step),
            IconD2FileU10m::get_file_url(step),
            IconD2FileV10m::get_file_url(step),
            IconD2FileVmax10m::get_file_url(step)
        ];
    }
}
