use crate::dwd::dwd_files::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;

pub struct IconD2FileClc;

const DWD_ICON_D2_CLC_FILE_PREFIX: &str = "/clc/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_CLC_FILE_SUFFIX: &str = "_clc.grib2.bz2";


impl IconD2FileClc {
    pub fn get_file_url(forecast_step: &DwdForecastStep, level: usize) -> String {
        return IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_CLC_FILE_PREFIX,
            DWD_ICON_D2_CLC_FILE_SUFFIX,
            level,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd::dwd_files::icon_d2_file_clc::IconD2FileClc;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 12, 22), IconD2ForecastRunName::Run00, 0);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/clc/icon-d2_germany_regular-lat-lon_model-level_2022122200_000_65_clc.grib2.bz2";

        let result = IconD2FileClc::get_file_url(&forecast_step, 65);

        assert_eq!(expected, result);
    }
}
