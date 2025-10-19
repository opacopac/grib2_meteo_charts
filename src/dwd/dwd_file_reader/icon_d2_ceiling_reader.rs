use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;


pub struct IconD2CeilingReader;


const DWD_ICON_D2_CEILING_FILE_PREFIX: &str = "/ceiling/icon-d2_germany_regular-lat-lon_single-level_";
const DWD_ICON_D2_CEILING_FILE_SUFFIX: &str = "_2d_ceiling.grib2.bz2";
const MISSING_VALUE: f32 = -1.0;


impl IconD2CeilingReader {
    pub fn read_grid_from_file(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let grid = FileToGridConverter::read_rectangular_grid_from_file(
            Self::get_file_url2(fc_run, fc_step).as_str(),
            MISSING_VALUE,
        )?;

        Ok(grid)
    }


    pub fn get_file_url(forecast_step: &DwdForecastStep) -> String {
        IconD2File::get_single_level_file_url(
            DWD_ICON_D2_CEILING_FILE_PREFIX,
            DWD_ICON_D2_CEILING_FILE_SUFFIX,
            forecast_step,
        )
    }


    pub fn get_file_url2(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
    ) -> String {
        IconD2File::get_single_level_file_url2(
            DWD_ICON_D2_CEILING_FILE_PREFIX,
            DWD_ICON_D2_CEILING_FILE_SUFFIX,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_ceiling_reader::IconD2CeilingReader;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use chrono::NaiveDate;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2022, 6, 19).unwrap(),
            IconD2ForecastRunName::Run00,
            0,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/ceiling/icon-d2_germany_regular-lat-lon_single-level_2022061900_000_2d_ceiling.grib2.bz2";

        let result = IconD2CeilingReader::get_file_url(&forecast_step);

        assert_eq!(expected, result);
    }
}
