use crate::dwd::dwd_files::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;


pub struct IconD2FileClctMod;


const DWD_ICON_D2_CLCT_MOD_FILE_PREFIX: &str = "/clct_mod/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_CLCT_MOD_FILE_SUFFIX: &str = "_2d_clct_mod.grib2.bz2";


impl IconD2FileClctMod {
    pub fn get_file_url(forecast_step: &DwdForecastStep) -> String {
        IconD2File::get_single_level_file_url(
            DWD_ICON_D2_CLCT_MOD_FILE_PREFIX,
            DWD_ICON_D2_CLCT_MOD_FILE_SUFFIX,
            forecast_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_files::icon_d2_file_clct_mod::IconD2FileClctMod;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 6, 19).unwrap(),
            IconD2ForecastRunName::Run00, 0,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/clct_mod/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_clct_mod.grib2.bz2";

        let result = IconD2FileClctMod::get_file_url(&forecast_step);

        assert_eq!(expected, result);
    }
}
