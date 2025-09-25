use std::ops::RangeInclusive;

use log::info;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::dwd::common::dwd_error::DwdError;
use crate::dwd::dwd_files::icon_d2_file_u::IconD2FileU;
use crate::dwd::forecast_run::dwd_forecast_step::DwdForecastStep;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;

pub struct IconD2UReader;

impl IconD2UReader {
    const MISSING_VALUE: u8 = 0xFF;
    const KNOTS_PER_MPS: f32 = 1.94384; // TODO: move to common place

    pub fn read_u_grids(
        fc_step: &DwdForecastStep,
        vertical_level_range: RangeInclusive<u8>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, DwdError> {
        let transform_fn = |x: f32| {
            (x * Self::KNOTS_PER_MPS + 128.0)
                .round()
                .min(254.0)
                .max(0.0) as u8
        };

        info!("reading u grids...");

        let u_grids = vertical_level_range
            .into_par_iter()
            .map(|level| {
                info!("reading clc layers for level {}", level);
                let url = IconD2FileU::get_file_url(&fc_step, level as usize);
                let grid = FileToGridConverter::read_rectangular_grid_from_file_and_transform(
                    &url,
                    Self::MISSING_VALUE,
                    transform_fn,
                )?;

                Ok(grid)
            })
            .collect();

        info!("reading u grids done");

        return u_grids;
    }
}
