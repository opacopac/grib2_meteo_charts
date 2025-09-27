use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_hhl::IconD2FileHhl;
use crate::dwd::forecast_run::dwd_forecast_run::DwdForecastRun;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::physics::length::Length;
use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;


pub struct IconD2HhlReader;


impl IconD2HhlReader {
    const MISSING_VALUE: u8 = 0;


    pub fn read_hhl_grids(
        forecast_run: &DwdForecastRun,
        vertical_level_range: &RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, DwdError> {
        let transform_fn = |x| (Length::from_meters_to_feet(x) / 100.0) as u8;

        info!("reading hhl grids...");

        let hhl_grids = vertical_level_range.clone()
            .into_par_iter()
            .map(|level| {
                info!("reading hhl layers for level {}", level);
                let url = IconD2FileHhl::get_file_url(&forecast_run, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(&url, Self::MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading hhl grids done.");

        return hhl_grids;
    }
}
