use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;

pub struct IconD2File;

pub const DWD_ICON_D2_BASE_URL: &str = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/";
pub const DWD_DATE_FORMAT: &str = "%Y%m%d";


impl IconD2File {
    pub fn get_file_url(
        file_prefix: &str,
        file_suffix: &str,
        forecast_step: &DwdForecastStep
    ) -> String {
        let date_str = forecast_step.run.start_date.format(DWD_DATE_FORMAT).to_string();
        let step_str = format!("{:03}", forecast_step.step);
        let run_str = &forecast_step.run.run_name.get_name();

        return format!(
            "{}{}{}{}{}_{}{}",
            DWD_ICON_D2_BASE_URL,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            file_suffix
        );
    }
}
