use crate::dwd::dwd_file_reader::dwd_icon_file::DwdIconFile;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconWwReader;


impl DwdIconWwReader {
    const DWD_ICON_D2_WW_FILE_PREFIX: &str = "/ww/icon-d2_germany_regular-lat-lon_single-level_";
    const DWD_ICON_EU_WW_FILE_PREFIX: &str = "/ww/icon-eu_europe_regular-lat-lon_single-level_";
    const DWD_ICON_D2_WW_FILE_SUFFIX: &str = "_2d_ww.grib2.bz2";
    const DWD_ICON_EU_WW_FILE_SUFFIX: &str = "_WW.grib2.bz2";
    const MISSING_VALUE: f32 = -1.0;


    pub fn read_grid_from_file(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let file_url = &Self::get_file_url(fc_run, fc_step);
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            file_url,
            Self::MISSING_VALUE,
        )?;

        Ok(grid)
    }


    pub fn get_file_url(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> String {
        let (file_prefix, file_suffix) = Self::get_file_prefix_suffix(fc_run);

        DwdIconFile::get_single_level_file_url(
            file_prefix,
            file_suffix,
            fc_run,
            fc_step,
        )
    }


    fn get_file_prefix_suffix(fc_run: &MeteoForecastRun) -> (&str, &str) {
        match fc_run.get_model() {
            MeteoForecastModel::IconD2 => (Self::DWD_ICON_D2_WW_FILE_PREFIX, Self::DWD_ICON_D2_WW_FILE_SUFFIX),
            MeteoForecastModel::IconEu => (Self::DWD_ICON_EU_WW_FILE_PREFIX, Self::DWD_ICON_EU_WW_FILE_SUFFIX),
            _ => panic!("Unsupported model for WW data: {}", fc_run.get_model()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::dwd_icon_ww_reader::DwdIconWwReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_icon_d2_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2022, 06, 19).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(1, "".to_string()); // TODO: get rid of this...

        // when
        let result = DwdIconWwReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/ww/icon-d2_germany_regular-lat-lon_single-level_2022061900_001_2d_ww.grib2.bz2";
        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_icon_eu_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconEu,
            NaiveDate::from_ymd_opt(2025, 12, 01).unwrap(),
            "06".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(63, "".to_string()); // TODO: get rid of this...

        // when
        let result = DwdIconWwReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-eu/grib/06/ww/icon-eu_europe_regular-lat-lon_single-level_2025120106_063_WW.grib2.bz2";
        assert_eq!(expected, result);
    }
}
