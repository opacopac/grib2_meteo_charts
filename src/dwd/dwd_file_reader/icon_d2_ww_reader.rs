use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;


pub struct IconD2WwReader;


const DWD_ICON_D2_WW_FILE_PREFIX: &str = "/ww/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_WW_FILE_SUFFIX: &str = "_2d_ww.grib2.bz2";
const MISSING_VALUE: f32 = -1.0;


impl IconD2WwReader {
    pub fn read_grid_from_file(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            Self::get_file_url(fc_run, fc_step).as_str(),
            MISSING_VALUE,
        )?;

        Ok(grid)
    }


    pub fn get_file_url(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
    ) -> String {
        IconD2File::get_single_level_file_url(
            DWD_ICON_D2_WW_FILE_PREFIX,
            DWD_ICON_D2_WW_FILE_SUFFIX,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_ww_reader::IconD2WwReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
    use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url2() {
        // given
        let fc_run = MeteoForecastRun2::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2022, 06, 19).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRun2Step::new(1, "".to_string()); // TODO: get rid of this...

        // when
        let result = IconD2WwReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/ww/icon-d2_germany_regular-lat-lon_single-level_2022061900_001_2d_ww.grib2.bz2";
        assert_eq!(expected, result);
    }
}
