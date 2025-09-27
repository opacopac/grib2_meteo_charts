use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_swiss::common::meteo_swiss_error::MeteoSwissError;
use crate::physics::speed::Speed;
use log::info;
use std::ops::RangeInclusive;


pub struct IconChVReader;


impl IconChVReader {
    const MISSING_VALUE: u8 = 0xFF;


    pub fn read_grids(
        file_url: &str,
        unstructured_grid: &UnstructuredGrid,
        vertical_level_range: Option<&RangeInclusive<usize>>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, MeteoSwissError> {
        info!("reading v grids...");

        let regular_grids = FileToGridConverter::read_multi_unstructured_grids_from_file_and_transform(
            file_url,
            Self::MISSING_VALUE,
            Self::transform_values,
            unstructured_grid,
            vertical_level_range,
        )?;

        info!("reading v grids done.");

        Ok(regular_grids)
    }


    fn transform_values(value: f32) -> u8 {
        (Speed::from_mps_to_knots(value) + 128.0)
            .round()
            .min(254.0)
            .max(0.0) as u8
    }
}
