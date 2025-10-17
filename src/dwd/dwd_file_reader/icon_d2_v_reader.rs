use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run2::MeteoForecastRun2;
use crate::meteo_common::meteo_forecast_run2_step::MeteoForecastRun2Step;
use crate::physics::speed::Speed;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;


pub struct IconD2VReader;


const DWD_ICON_D2_V_FILE_PREFIX: &str = "/v/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_V_FILE_SUFFIX: &str = "_v.grib2.bz2";
const MISSING_VALUE: u8 = 0xFF;


impl IconD2VReader {
    pub fn read_v_grids(
        fc_step: &DwdForecastStep,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
        let transform_fn = |x: f32| (Speed::from_mps_to_knots(x) + 128.0).round().min(254.0).max(0.0) as u8;

        info!("reading v grids...");

        let v_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = Self::get_file_url(&fc_step, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(&url, MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading v grids done");

        v_grids
    }


    pub fn read_v_grids2(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
        let transform_fn = |x: f32| (Speed::from_mps_to_knots(x) + 128.0).round().min(254.0).max(0.0) as u8;

        info!("reading v grids...");

        let v_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = Self::get_file_url2(fc_run, fc_step, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(&url, MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading v grids done");

        v_grids
    }


    pub fn get_file_url(forecast_step: &DwdForecastStep, level: usize) -> String {
        IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_V_FILE_PREFIX,
            DWD_ICON_D2_V_FILE_SUFFIX,
            level,
            forecast_step,
        )
    }


    pub fn get_file_url2(
        fc_run: &MeteoForecastRun2,
        fc_step: &MeteoForecastRun2Step,
        level: usize,
    ) -> String {
        IconD2File::get_multi_level_file_url2(
            DWD_ICON_D2_V_FILE_PREFIX,
            DWD_ICON_D2_V_FILE_SUFFIX,
            level,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_v_reader::IconD2VReader;
    use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
    use crate::dwd::forecast_run::dwd_model_type::DwdModelType;
    use crate::dwd::forecast_run::icon_d2_forecast_run_name::IconD2ForecastRunName;
    use chrono::NaiveDate;

    #[test]
    fn it_creates_the_correct_file_url() {
        let forecast_step = DwdForecastStep::new(
            DwdModelType::IconD2,
            NaiveDate::from_ymd_opt(2023, 03, 21).unwrap(),
            IconD2ForecastRunName::Run00,
            43,
        );
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/v/icon-d2_germany_regular-lat-lon_model-level_2023032100_043_48_v.grib2.bz2";

        let result = IconD2VReader::get_file_url(&forecast_step, 48);

        assert_eq!(expected, result);
    }
}
