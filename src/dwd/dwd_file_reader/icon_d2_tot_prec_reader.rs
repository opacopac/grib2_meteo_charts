use crate::dwd::dwd_file_reader::icon_d2_file::DwdIconFile;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconD2TotPrecReader;

const DWD_ICON_D2_TOT_PREC_FILE_PREFIX: &str = "/tot_prec/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_TOT_PREC_FILE_SUFFIX: &str = "_2d_tot_prec.grib2.bz2";
const MISSING_VALUE: f32 = -1.0;


impl IconD2TotPrecReader {
    pub fn read_grid_from_file(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            Self::get_file_url(fc_run, fc_step).as_str(),
            MISSING_VALUE,
        )?;

        Ok(grid)
    }


    pub fn get_file_url(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
    ) -> String {
        DwdIconFile::get_single_level_file_url(
            DWD_ICON_D2_TOT_PREC_FILE_PREFIX,
            DWD_ICON_D2_TOT_PREC_FILE_SUFFIX,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_tot_prec_reader::IconD2TotPrecReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2022, 06, 19).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(0, "".to_string()); // TODO: get rid of this...

        // when
        let result = IconD2TotPrecReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/tot_prec/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_tot_prec.grib2.bz2";
        assert_eq!(expected, result);
    }
}
