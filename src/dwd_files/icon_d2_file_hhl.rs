use crate::dwd_files::icon_d2_file::IconD2File;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;

pub struct IconD2FileHhl;

const DWD_ICON_D2_HHL_FILE_PREFIX: &str = "/hhl/icon-d2_germany_regular-lat-lon_time-invariant_";
const DWD_ICON_D2_HHL_FILE_SUFFIX: &str = "_hhl.grib2.bz2";


impl IconD2FileHhl {
    pub fn get_file_url(forecast_step: &DwdForecastStep, level: usize) -> String {
        return IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_HHL_FILE_PREFIX,
            DWD_ICON_D2_HHL_FILE_SUFFIX,
            level,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd_files::icon_d2_file_hhl::IconD2FileHhl;
    use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
    use crate::dwd_forecast_runs::dwd_model_type::DwdModelType;
    use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 12, 22), IconD2ForecastRunName::Run00, 0);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/hhl/icon-d2_germany_regular-lat-lon_time-invariant_2022122200_000_10_hhl.grib2.bz2";

        let result = IconD2FileHhl::get_file_url(&forecast_step, 10);

        assert_eq!(expected, result);
    }
}
