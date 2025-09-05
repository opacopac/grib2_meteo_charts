use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use log::info;
use std::ops::RangeInclusive;

pub struct IconChClcReader;


impl IconChClcReader {
    const MISSING_VALUE: u8 = 0;


    pub fn read_grids(
        file_url: &str,
        unstructured_grid: &UnstructuredGrid,
        vertical_level_range: Option<RangeInclusive<usize>>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, MeteoSwissError> {
        info!("reading clc grids...");

        let regular_grids = FileToGridConverter::read_multi_unstructured_grids_from_file_and_transform(
            file_url,
            Self::MISSING_VALUE,
            Self::transform_values,
            unstructured_grid,
            vertical_level_range,
        )?;

        info!("reading clc grids done.");

        Ok(regular_grids)
    }


    fn transform_values(value: f32) -> u8 {
        value as u8
    }
}
