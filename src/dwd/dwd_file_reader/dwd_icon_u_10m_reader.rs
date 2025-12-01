use crate::dwd::dwd_file_reader::dwd_icon_file::DwdIconFile;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconU10mReader;


impl DwdIconU10mReader {
    const DWD_ICON_D2_U_10M_FILE_PREFIX: &str = "/u_10m/icon-d2_germany_regular-lat-lon_single-level_";
    const DWD_ICON_EU_U_10M_FILE_PREFIX: &str = "/u_10m/icon-eu_europe_regular-lat-lon_single-level_";
    const DWD_ICON_D2_U_10M_FILE_SUFFIX: &str = "_2d_u_10m.grib2.bz2";
    const DWD_ICON_EU_U_10M_FILE_SUFFIX: &str = "_U_10M.grib2.bz2";
    const MISSING_VALUE: f32 = -1.0;


    pub fn read_grid_from_file(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let missing_value = Self::MISSING_VALUE;
        let file_url = &Self::get_file_url(fc_run, fc_step);
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            file_url,
            missing_value,
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
            MeteoForecastModel::IconD2 => (Self::DWD_ICON_D2_U_10M_FILE_PREFIX, Self::DWD_ICON_D2_U_10M_FILE_SUFFIX),
            MeteoForecastModel::IconEu => (Self::DWD_ICON_EU_U_10M_FILE_PREFIX, Self::DWD_ICON_EU_U_10M_FILE_SUFFIX),
            _ => panic!("Unsupported model for T 2M data: {}", fc_run.get_model()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::dwd_icon_u_10m_reader::DwdIconU10mReader;
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
        let fc_step = MeteoForecastRunStep::new(0, "".to_string()); // TODO: get rid of this...

        // when
        let result = DwdIconU10mReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/u_10m/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_u_10m.grib2.bz2";
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
        let fc_step = MeteoForecastRunStep::new(20, "".to_string()); // TODO: get rid of this...

        // when
        let result = DwdIconU10mReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-eu/grib/06/u_10m/icon-eu_europe_regular-lat-lon_single-level_2025120106_020_U_10M.grib2.bz2";
        assert_eq!(expected, result);
    }
}
