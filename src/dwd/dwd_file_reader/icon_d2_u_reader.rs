use crate::dwd::dwd_file_reader::icon_d2_file::IconD2File;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
use crate::physics::speed::Speed;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;


pub struct IconD2UReader;


const DWD_ICON_D2_U_FILE_PREFIX: &str = "/u/icon-d2_germany_regular-lat-lon_model-level_";
const DWD_ICON_D2_U_FILE_SUFFIX: &str = "_u.grib2.bz2";
const MISSING_VALUE: u8 = 0xFF;


impl IconD2UReader {
    pub fn read_u_grids(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
        let transform_fn = |x: f32| {
            (Speed::from_mps_to_knots(x) + 128.0)
                .round()
                .min(254.0)
                .max(0.0) as u8
        };

        info!("reading u grids for {fc_run}...");

        let u_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading u layers for forecast step {fc_step}, level {level}");
                let url = Self::get_file_url(fc_run, fc_step, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(
                    &url,
                    MISSING_VALUE,
                    transform_fn,
                )?;

                Ok(grid)
            })
            .collect();

        info!("reading u grids done");

        u_grids
    }


    pub fn get_file_url(
        fc_run: &MeteoForecastRun,
        fc_step: &MeteoForecastRunStep,
        level: usize,
    ) -> String {
        IconD2File::get_multi_level_file_url(
            DWD_ICON_D2_U_FILE_PREFIX,
            DWD_ICON_D2_U_FILE_SUFFIX,
            level,
            fc_run,
            fc_step,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::icon_d2_u_reader::IconD2UReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2023, 03, 21).unwrap(),
            "00".to_string(),
        );
        let fc_step = MeteoForecastRunStep::new(4, "".to_string()); // TODO: get rid of this...

        // when
        let result = IconD2UReader::get_file_url(&fc_run, &fc_step, 11);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/u/icon-d2_germany_regular-lat-lon_model-level_2023032100_004_11_u.grib2.bz2";
        assert_eq!(expected, result);
    }
}