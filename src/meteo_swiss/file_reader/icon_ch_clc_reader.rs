use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_vertical_cloud_layer::MeteoVerticalCloudLayer;
use log::info;
use std::ops::RangeInclusive;
use crate::meteo_common::meteo_forecast_run_step::MeteoForecastRunStep;


pub struct IconChClcReader;


impl IconChClcReader {
    const MISSING_VALUE: u8 = 0;


    pub fn read_layer_from_file(
        clc_step: &MeteoForecastRunStep,
        unstructured_grid: &UnstructuredGrid,
        hhl_grids: &Vec<LatLonValueGrid<u8>>,
        vertical_level_range: Option<&RangeInclusive<usize>>,
    ) -> Result<MeteoVerticalCloudLayer, Grib2Error> {
        let regular_clc_grids = Self::read_grids(clc_step.get_file_url(), unstructured_grid, vertical_level_range)?;
        let layer = MeteoVerticalCloudLayer::new(hhl_grids.clone(), regular_clc_grids);

        Ok(layer)
    }


    pub fn read_grids(
        file_url: &str,
        unstructured_grid: &UnstructuredGrid,
        vertical_level_range: Option<&RangeInclusive<usize>>,
    ) -> Result<Vec<LatLonValueGrid<u8>>, Grib2Error> {
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
