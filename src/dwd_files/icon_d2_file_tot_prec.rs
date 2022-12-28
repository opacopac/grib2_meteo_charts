use crate::dwd_files::icon_d2_file::IconD2File;
use crate::dwd_files::icon_d2_file_to_grid_converter::IconD2FileToGridConverter;
use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2FileTotPrec;

pub const DWD_ICON_D2_TOT_PREC_FILE_PREFIX: &str = "/tot_prec/icon-d2_germany_regular-lat-lon_single-level_";
pub const DWD_ICON_D2_TOT_PREC_FILE_SUFFIX: &str = "_2d_tot_prec.grib2.bz2";


impl IconD2FileTotPrec {
    pub fn read_grid_from_file(fc_step: &DwdForecastStep) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let url = Self::get_file_url(&fc_step);

        return IconD2FileToGridConverter::read_grid_from_file(&url);
    }


    pub fn get_file_url(forecast_step: &DwdForecastStep) -> String {
        return IconD2File::get_single_level_file_url(
            DWD_ICON_D2_TOT_PREC_FILE_PREFIX,
            DWD_ICON_D2_TOT_PREC_FILE_SUFFIX,
            forecast_step
        );
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dwd_files::icon_d2_file_tot_prec::IconD2FileTotPrec;
    use crate::dwd_forecast_runs::dwd_forecast_step::DwdForecastStep;
    use crate::dwd_forecast_runs::dwd_model_type::DwdModelType;
    use crate::dwd_forecast_runs::icon_d2_forecast_run_name::IconD2ForecastRunName;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(DwdModelType::IconD2, NaiveDate::from_ymd(2022, 6, 19), IconD2ForecastRunName::Run00, 0);
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/tot_prec/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_tot_prec.grib2.bz2";

        let result = IconD2FileTotPrec::get_file_url(&forecast_step);

        assert_eq!(expected, result);
    }
}
