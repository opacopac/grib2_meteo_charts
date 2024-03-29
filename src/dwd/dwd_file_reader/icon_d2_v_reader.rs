use std::ops::RangeInclusive;

use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_to_grid_converter::IconD2FileToGridConverter;
use crate::dwd::dwd_files::icon_d2_file_v::IconD2FileV;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2VReader;

impl IconD2VReader {
    const MISSING_VALUE: u8 = 0xFF;
    const KNOTS_PER_MPS: f32 = 1.94384; // TODO: move to common place


    pub fn read_v_grids(
        fc_step: &DwdForecastStep,
        vertical_level_range: RangeInclusive<u8>
    ) -> Result<Vec<LatLonValueGrid<u8>>, DwdError> {
        let transform_fn = |x: f32| (x * Self::KNOTS_PER_MPS + 128.0).round().min(254.0).max(0.0) as u8;

        info!("reading v grids...");

        let v_grids = vertical_level_range
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = IconD2FileV::get_file_url(&fc_step, level as usize);

                return IconD2FileToGridConverter::read_grid_from_file_and_convert(&url, Self::MISSING_VALUE, transform_fn);
            }).collect();

        info!("reading v grids done");

        return v_grids;
    }
}
