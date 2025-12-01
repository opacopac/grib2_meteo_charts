use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconFile;

pub const DWD_ICON_D2_BASE_URL: &str = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/";
pub const DWD_ICON_EU_BASE_URL: &str = "https://opendata.dwd.de/weather/nwp/icon-eu/grib/";
pub const DWD_DATE_FORMAT: &str = "%Y%m%d";


impl DwdIconFile {
    pub fn get_single_level_file_url(
        file_prefix: &str,
        file_suffix: &str,
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> String {
        let base_url = Self::get_base_url(fc_run);
        let date_str = fc_run.get_start_date().format(DWD_DATE_FORMAT).to_string();
        let step_str = format!("{:03}", fc_step.get_step_nr());
        let run_str = fc_run.get_name();

        format!(
            "{}{}{}{}{}_{}{}",
            base_url,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            file_suffix
        )
    }


    pub fn get_multi_level_file_url(
        file_prefix: &str,
        file_suffix: &str,
        level: usize,
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> String {
        let base_url = Self::get_base_url(fc_run);
        let date_str = fc_run.get_start_date().format(DWD_DATE_FORMAT).to_string();
        let step_str = format!("{:03}", fc_step.get_step_nr());
        let run_str = &fc_run.get_name();

        format!(
            "{}{}{}{}{}_{}_{}{}",
            base_url,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            level,
            file_suffix
        )
    }


    pub fn get_multi_level_time_invariant_file_url(
        file_prefix: &str,
        file_suffix: &str,
        level: usize,
        fc_run: &MeteoForecastRun,
    ) -> String {
        let base_url = Self::get_base_url(fc_run);
        let date_str = fc_run.get_start_date().format(DWD_DATE_FORMAT).to_string();
        let run_str = fc_run.get_name();
        let step_str = match fc_run.get_model() {
            MeteoForecastModel::IconD2 => "000_",
            MeteoForecastModel::IconEu => "",
            _ => panic!("Unsupported model for DWD ICON time-invariant file URL generation: {}", fc_run.get_model()),
        };


        format!(
            "{}{}{}{}{}_{}{}{}",
            base_url,
            run_str,
            file_prefix,
            date_str,
            run_str,
            step_str,
            level,
            file_suffix
        )
    }


    fn get_base_url(fc_run: &MeteoForecastRun) -> &str {
        match fc_run.get_model() {
            MeteoForecastModel::IconD2 => DWD_ICON_D2_BASE_URL,
            MeteoForecastModel::IconEu => DWD_ICON_EU_BASE_URL,
            _ => panic!("Unsupported model for DWD ICON file URL generation: {}", fc_run.get_model()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::dwd_icon_file::DwdIconFile;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_single_level_icon_d2_file_url() {
        // given
        let file_prefix = "/t_2m/icon-d2_germany_regular-lat-lon_single-level_";
        let file_suffix = "_2d_t_2m.grib2.bz2";
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2025, 11, 20).unwrap(),
            "06".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(13, "".to_string());

        // when
        let result = DwdIconFile::get_single_level_file_url(file_prefix, file_suffix, &fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/06/t_2m/icon-d2_germany_regular-lat-lon_single-level_2025112006_013_2d_t_2m.grib2.bz2";
        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_multi_level_icon_eu_file_url() {
        // given
        let file_prefix = "/clc/icon-eu_europe_regular-lat-lon_model-level_";
        let file_suffix = "_CLC.grib2.bz2";
        let level = 32;
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconEu,
            NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
            "06".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(0, "".to_string());

        // when
        let result = DwdIconFile::get_multi_level_file_url(file_prefix, file_suffix, level, &fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-eu/grib/06/clc/icon-eu_europe_regular-lat-lon_model-level_2025120106_000_32_CLC.grib2.bz2";
        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_multi_level_time_invariant_icon_d2_file_url() {
        // given
        let file_prefix = "/hhl/icon-d2_germany_icosahedral_time-invariant_";
        let file_suffix = "_hhl.grib2.bz2";
        let level = 66;
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2025, 11, 20).unwrap(),
            "06".to_string(),
        );

        // when
        let result = DwdIconFile::get_multi_level_time_invariant_file_url(file_prefix, file_suffix, level, &fc_run);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/06/hhl/icon-d2_germany_icosahedral_time-invariant_2025112006_000_66_hhl.grib2.bz2";
        assert_eq!(expected, result);
    }
}
