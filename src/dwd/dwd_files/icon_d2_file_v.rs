use crate::dwd::dwd_files::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;

pub struct IconD2FileV;

const DWD_ICON_D2_V_FILE_PREFIX: &str = "/v/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_V_FILE_SUFFIX: &str = "_v.grib2.bz2";


impl IconD2FileV {
    pub fn get_file_url(forecast_step: &DwdForecastStep, level: usize) -> String {
        return IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_V_FILE_PREFIX,
            DWD_ICON_D2_V_FILE_SUFFIX,
            level,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd::dwd_files::icon_d2_file_v::IconD2FileV;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2023, 03, 21), IconD2ForecastRunName::Run00, 43);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/v/icon-d2_germany_regular-lat-lon_model-level_2023032100_043_48_v.grib2.bz2";

        let result = IconD2FileV::get_file_url(&forecast_step, 48);

        assert_eq!(expected, result);
    }
}
