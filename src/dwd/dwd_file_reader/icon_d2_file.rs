use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;

pub struct IconD2File;

pub const DWD_ICON_D2_BASE_URL: &str = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/";
pub const DWD_DATE_FORMAT: &str = "%Y%m%d";


impl IconD2File {
    pub fn get_single_level_file_url(
        file_prefix: &str,
        file_suffix: &str,
        forecast_step: &DwdForecastStep,
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


    pub fn get_multi_level_file_url(
        file_prefix: &str,
        file_suffix: &str,
        level: usize,
        forecast_step: &DwdForecastStep,
    ) -> String {
        let date_str = forecast_step.run.start_date.format(DWD_DATE_FORMAT).to_string();
        let step_str = format!("{:03}", forecast_step.step);
        let run_str = &forecast_step.run.run_name.get_name();

        return format!(
            "{}{}{}{}{}_{}_{}{}",
            DWD_ICON_D2_BASE_URL,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            level,
            file_suffix
        );
    }


    pub fn get_multi_level_file_url2(
        file_prefix: &str,
        file_suffix: &str,
        level: usize,
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
    ) -> String {
        let date_str = fc_run.get_start_date().format(DWD_DATE_FORMAT).to_string();
        let step_str = format!("{:03}", fc_step.get_step_nr());
        let run_str = &fc_run.get_name();

        return format!(
            "{}{}{}{}{}_{}_{}{}",
            DWD_ICON_D2_BASE_URL,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            level,
            file_suffix
        );
    }


    pub fn get_multi_level_time_invariant_file_url(
        file_prefix: &str,
        file_suffix: &str,
        level: usize,
        forecast_run: &DwdForecastRun,
    ) -> String {
        let date_str = forecast_run.start_date.format(DWD_DATE_FORMAT).to_string();
        let step_str = "000";
        let run_str = forecast_run.run_name.get_name();

        return format!(
            "{}{}{}{}{}_{}_{}{}",
            DWD_ICON_D2_BASE_URL,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            level,
            file_suffix
        );
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
    use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_single_level_file_url() {
        let file_prefix = "/xxx/PREFIX_";
        let file_suffix = "_SUFFIX.grib2.bz2";
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 6, 19).unwrap(),
            IconD2ForecastRunName::Run00,
            13,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/xxx/PREFIX_2022061900_013_SUFFIX.grib2.bz2";

        let result = IconD2File::get_single_level_file_url(file_prefix, file_suffix, &forecast_step);

        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_multi_level_file_url() {
        let file_prefix = "/xxx/PREFIX_";
        let file_suffix = "_SUFFIX.grib2.bz2";
        let level = 66;
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 6, 19).unwrap(),
            IconD2ForecastRunName::Run00,
            12,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/xxx/PREFIX_2022061900_012_66_SUFFIX.grib2.bz2";

        let result = IconD2File::get_multi_level_file_url(file_prefix, file_suffix, level, &forecast_step);

        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_multi_level_time_invariant_file_url() {
        let file_prefix = "/xxx/PREFIX_";
        let file_suffix = "_SUFFIX.grib2.bz2";
        let level = 66;
        let forecast_run = DwdForecastRun::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 6, 19).unwrap(),
            IconD2ForecastRunName::Run00,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/xxx/PREFIX_2022061900_000_66_SUFFIX.grib2.bz2";

        let result = IconD2File::get_multi_level_time_invariant_file_url(file_prefix, file_suffix, level, &forecast_run);

        assert_eq!(expected, result);
    }
}
