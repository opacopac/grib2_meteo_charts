use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_file_reader::dwd_icon_ceiling_reader::DwdIconCeilingReader;
use crate::dwd::dwd_file_reader::dwd_icon_clc_reader::DwdIconClcReader;
use crate::dwd::dwd_file_reader::icon_d2_clct_mod_reader::IconD2ClctModReader;
use crate::dwd::dwd_file_reader::icon_d2_t_2m_reader::IconD2T2mReader;
use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
use crate::dwd::dwd_file_reader::icon_d2_u_10m_reader::IconD2U10mReader;
use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
use crate::dwd::dwd_file_reader::icon_d2_v_10m_reader::IconD2V10mReader;
use crate::dwd::dwd_file_reader::icon_d2_vmax_10m_reader::IconD2Vmax10mReader;
use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use chrono;
use chrono::{Duration, Utc};
use std::ops::Add;

pub struct IconD2ForecastRunFinder;


impl IconD2ForecastRunFinder {
    pub fn find_latest_forecast_run() -> Result<MeteoForecastRun, DwdError> {
        let date_today = Utc::now().date_naive();

        // return Ok(IconD2ForecastRun::new(date_today, IconD2ForecastRunName::Run12));
        let last_step = MeteoForecastModel::IconD2.get_step_range().end().clone();

        // check each run
        for run in IconD2ForecastRunName::get_all_reversed() {
            let fc_run = MeteoForecastRun::new(MeteoForecastModel::IconD2, date_today, run.get_name());
            let fc_step = MeteoForecastRunStep::new(last_step, "".to_string());
            let probe_file_names = Self::get_probe_file_names(&fc_run, &fc_step);

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
                return Ok(fc_run);
            }
        }

        // TODO: check if yesterday's files really exist
        let date_yesterday = Utc::now().date_naive().add(Duration::days(-1));
        let fc_run_yesterday = MeteoForecastRun::new(MeteoForecastModel::IconD2, date_yesterday, IconD2ForecastRunName::Run21.get_name());

        Ok(fc_run_yesterday)
    }


    pub fn get_probe_file_names(fc_run: &MeteoForecastRun, fc_step: &MeteoForecastRunStep) -> Vec<String> {
        vec![
            IconD2ClctModReader::get_file_url(fc_run, fc_step),
            IconD2TotPrecReader::get_file_url(fc_run, fc_step),
            DwdIconCeilingReader::get_file_url(fc_run, fc_step),
            IconD2WwReader::get_file_url(fc_run, fc_step),
            IconD2U10mReader::get_file_url(fc_run, fc_step),
            IconD2V10mReader::get_file_url(fc_run, fc_step),
            IconD2Vmax10mReader::get_file_url(fc_run, fc_step),
            IconD2T2mReader::get_file_url(fc_run, fc_step),
            DwdIconClcReader::get_file_url(fc_run, fc_step, 65),
            IconD2UReader::get_file_url(fc_run, fc_step, 65)
        ]
    }
}
