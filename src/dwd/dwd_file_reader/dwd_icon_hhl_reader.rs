use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_file_reader::dwd_icon_file::DwdIconFile;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
use crate::physics::length::Length;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;
use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;


pub struct DwdIconHhlReader;


impl DwdIconHhlReader {
    const DWD_ICON_D2_HHL_FILE_PREFIX: &str = "/hhl/icon-d2_germany_regular-lat-lon_time-invariant_";
    const DWD_ICON_EU_HHL_FILE_PREFIX: &str = "/hhl/icon-eu_europe_regular-lat-lon_time-invariant_";
    const DWD_ICON_D2_HHL_FILE_SUFFIX: &str = "_hhl.grib2.bz2";
    const DWD_ICON_EU_HHL_FILE_SUFFIX: &str = "_HHL.grib2.bz2";
    const MISSING_VALUE: u8 = 0;


    pub fn read_hhl_grids(
        fc_run: &MeteoForecastRun,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, DwdError> {
        let transform_fn = |x| (Length::from_meters_to_feet(x) / 100.0) as u8;

        info!("reading hhl grids for {fc_run}...");

        let hhl_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading hhl layers for level {level}");
                let url = Self::get_file_url(&fc_run, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(&url, Self::MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading hhl grids done for {fc_run}.");

        hhl_grids
    }


    fn get_file_url(forecast_run: &MeteoForecastRun, level: usize) -> String {
        let (file_prefix, file_suffix) = Self::get_file_prefix_suffix(forecast_run);

        DwdIconFile::get_multi_level_time_invariant_file_url(
            file_prefix,
            file_suffix,
            level,
            forecast_run,
        )
    }


    fn get_file_prefix_suffix(fc_run: &MeteoForecastRun) -> (&str, &str) {
        match fc_run.get_model() {
            MeteoForecastModel::IconD2 => (Self::DWD_ICON_D2_HHL_FILE_PREFIX, Self::DWD_ICON_D2_HHL_FILE_SUFFIX),
            MeteoForecastModel::IconEu => (Self::DWD_ICON_EU_HHL_FILE_PREFIX, Self::DWD_ICON_EU_HHL_FILE_SUFFIX),
            _ => panic!("Unsupported model for CLCT MOD data: {}", fc_run.get_model()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dwd::dwd_file_reader::dwd_icon_hhl_reader::DwdIconHhlReader;
    use crate::meteo_common::meteo_forecast_model::MeteoForecastModel;
    use crate::meteo_common::meteo_forecast_run::MeteoForecastRun;
    use chrono::NaiveDate;


    #[test]
    fn it_creates_the_correct_icon_dw_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconD2,
            NaiveDate::from_ymd_opt(2022, 12, 22).unwrap(),
            "00".to_string(),
        );

        // when
        let result = DwdIconHhlReader::get_file_url(&fc_run, 10);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-d2/grib/00/hhl/icon-d2_germany_regular-lat-lon_time-invariant_2022122200_000_10_hhl.grib2.bz2";
        assert_eq!(expected, result);
    }


    #[test]
    fn it_creates_the_correct_icon_eu_file_url() {
        // given
        let fc_run = MeteoForecastRun::new(
            MeteoForecastModel::IconEu,
            NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
            "06".to_string(),
        );

        // when
        let result = DwdIconHhlReader::get_file_url(&fc_run, 9);

        // then
        let expected = "https://opendata.dwd.de/weather/nwp/icon-eu/grib/06/hhl/icon-eu_europe_regular-lat-lon_time-invariant_2025120106_9_HHL.grib2.bz2";
        assert_eq!(expected, result);
    }
}
