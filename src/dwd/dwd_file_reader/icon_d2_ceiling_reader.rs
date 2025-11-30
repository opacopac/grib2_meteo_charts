use crate::dwd::dwd_file_reader::icon_d2_file::DwdIconFile;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct DwdIconCeilingReader;


const DWD_ICON_D2_CEILING_FILE_PREFIX: &str = "/ceiling/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_CEILING_FILE_SUFFIX: &str = "_2d_ceiling.grib2.bz2";
const DWD_ICON_EU_CEILING_FILE_PREFIX: &str = "/ceiling/icon-eu_europe_regular-lat-lon_single-level_";
const DWD_ICON_EU_CEILING_FILE_SUFFIX: &str = "_CEILING.grib2.bz2";
const MISSING_VALUE: f32 = -1.0;


impl DwdIconCeilingReader {
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
            MeteoForecastModel::IconD2 => {
                (DWD_ICON_D2_CEILING_FILE_PREFIX, DWD_ICON_D2_CEILING_FILE_SUFFIX)
            }
            MeteoForecastModel::IconEu => {
                (DWD_ICON_EU_CEILING_FILE_PREFIX, DWD_ICON_EU_CEILING_FILE_SUFFIX)
            }
            _ => panic!("Unsupported model for DWD ICON ceiling file URL generation"),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::DwdIconCeilingReader;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            crate::meteo_common::meteo_forecast_model::MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2025, 11, 20).unwrap(),
            "06".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(25, "".to_string());

        // when
        let result = DwdIconCeilingReader::get_file_url(&fc_run, &fc_step);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/06/ceiling/icon-d2_germany_regular-lat-lon_single-level_2025112006_025_2d_ceiling.grib2.bz2";
        assert_eq!(expected, result);
    }
}
