use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
use crate::dwd::dwd_file_reader::icon_d2_clc_reader::IconD2ClcReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
use chrono;
use chrono::{Duration, Utc};
use std::ops::Add;


pub struct IconD2ForecastRunFinder;


impl IconD2ForecastRunFinder {
    pub fn find_latest_forecast_run() -> Result<DwdForecastRun, DwdError> {
        let date_today = Utc::now().date_naive();

        // return Ok(IconD2ForecastRun::new(date_today, IconD2ForecastRunName::Run12));
        let last_step = DwdForecastStep::get_step_range().end().clone();

        // check each run
        for run in IconD2ForecastRunName::get_all_reversed() {
            let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, date_today, run, last_step);
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
        let date_yesterday = Utc::now().date_naive().add(Duration::days(-1));
        let fc_run = DwdForecastRun::new(DwdModelType::IconD2, date_yesterday, IconD2ForecastRunName::Run21);

        Ok(fc_run)
    }


    pub fn get_probe_file_names(step: &DwdForecastStep) -> Vec<String> {
        vec![
            IconD2ClctModReader::get_file_url(step),
            IconD2TotPrecReader::get_file_url(step),
            IconD2CeilingReader::get_file_url(step),
            IconD2WwReader::get_file_url(step),
            IconD2U10mReader::get_file_url(step),
            IconD2V10mReader::get_file_url(step),
            IconD2Vmax10mReader::get_file_url(step),
            IconD2T2mReader::get_file_url(step),
            IconD2ClcReader::get_file_url(step, 65),
            IconD2UReader::get_file_url(step, 65)
        ]
    }
}
