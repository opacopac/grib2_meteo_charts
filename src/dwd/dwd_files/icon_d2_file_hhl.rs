use crate::dwd::dwd_files::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;

pub struct IconD2FileHhl;

const DWD_ICON_D2_HHL_FILE_PREFIX: &str = "/hhl/icon-d2_germany_regular-lat-lon_time-invariant_";
const DWD_ICON_D2_HHL_FILE_SUFFIX: &str = "_hhl.grib2.bz2";


impl IconD2FileHhl {
    pub fn get_file_url(forecast_run: &DwdForecastRun, level: usize) -> String {
        return IconD2File::get_multi_level_time_invariant_file_url(
            DWD_ICON_D2_HHL_FILE_PREFIX,
            DWD_ICON_D2_HHL_FILE_SUFFIX,
            level,
            forecast_run
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd::dwd_files::icon_d2_file_hhl::IconD2FileHhl;
    use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_run = DwdForecastRun::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 12, 22), IconD2ForecastRunName::Run00);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/hhl/icon-d2_germany_regular-lat-lon_time-invariant_2022122200_000_10_hhl.grib2.bz2";

        let result = IconD2FileHhl::get_file_url(&forecast_run, 10);

        assert_eq!(expected, result);
    }
}
