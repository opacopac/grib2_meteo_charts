use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file::IconD2File;
use crate::dwd::dwd_files::icon_d2_file_to_grid_converter::IconD2FileToGridConverter;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2FileWw;

pub const DWD_ICON_D2_WW_FILE_PREFIX: &str = "/ww/icon-d2_germany_regular-lat-lon_single-level_";
pub const DWD_ICON_D2_WW_FILE_SUFFIX: &str = "_2d_ww.grib2.bz2";


impl IconD2FileWw {
    pub fn read_grid_from_file(fc_step: &DwdForecastStep) -> Result<LatLonValueGrid<f32>, DwdError> {
        let url = Self::get_file_url(&fc_step);

        return IconD2FileToGridConverter::read_grid_from_file(&url);
    }


    pub fn get_file_url(forecast_step: &DwdForecastStep) -> String {
        return IconD2File::get_single_level_file_url(
            DWD_ICON_D2_WW_FILE_PREFIX,
            DWD_ICON_D2_WW_FILE_SUFFIX,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd::dwd_files::icon_d2_file_ww::IconD2FileWw;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 6, 19), IconD2ForecastRunName::Run00, 1);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/ww/icon-d2_germany_regular-lat-lon_single-level_2022061900_001_2d_ww.grib2.bz2";

        let result = IconD2FileWw::get_file_url(&forecast_step);

        assert_eq!(expected, result);
    }
}
