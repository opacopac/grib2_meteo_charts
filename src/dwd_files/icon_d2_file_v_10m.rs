use crate::dwd_files::icon_d2_file::IconD2File;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;

pub struct IconD2FileV10m;

pub const DWD_ICON_D2_V_10M_FILE_PREFIX: &str = "/v_10m/icon-d2_germany_regular-lat-lon_single-level_";
pub const DWD_ICON_D2_V_10M_FILE_SUFFIX: &str = "_2d_v_10m.grib2.bz2";


impl IconD2FileV10m {
    pub fn get_file_url(forecast_step: &DwdForecastStep) -> String {
        return IconD2File::get_file_url(
            DWD_ICON_D2_V_10M_FILE_PREFIX,
            DWD_ICON_D2_V_10M_FILE_SUFFIX,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::dwd_files::icon_d2_file_v_10m::IconD2FileV10m;
    use crate::dwd_forecast_runs::dwd_model_type::DwdModelType;
    use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 6, 19), IconD2ForecastRunName::Run00, 0);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/v_10m/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_v_10m.grib2.bz2";

        let result = IconD2FileV10m::get_file_url(&forecast_step);

        assert_eq!(expected, result);
    }
}
