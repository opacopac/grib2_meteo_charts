use std::ops::RangeInclusive;

use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_clc::IconD2FileClc;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2ClcReader;

impl IconD2ClcReader {
    const MISSING_VALUE: u8 = 0;


    pub fn read_clc_grids(
        fc_step: &DwdForecastStep,
        vertical_level_range: RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, DwdError> {
        let transform_fn = |x| x as u8;

        info!("reading clc grids...");

        let clc_grids = vertical_level_range
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = IconD2FileClc::get_file_url(&fc_step, level as usize);
                let grid = FileToGridConverter::read_grid_from_file_and_convert(&url, Self::MISSING_VALUE, transform_fn)?;

                Ok(grid)
            }).collect();

        info!("reading clc grids done");

        return clc_grids;
    }
}
