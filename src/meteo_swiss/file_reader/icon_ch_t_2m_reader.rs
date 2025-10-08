use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;
use crate::meteo_chart::meteo_layer::meteo_temp_layer::MeteoTempLayer;


pub struct IconChT2mReader;


impl IconChT2mReader {
    const MISSING_VALUE: f32 = -1.0;


    pub fn read_layer_from_file(file_url: &str, unstructured_grid: &UnstructuredGrid) -> Result<MeteoTempLayer, Grib2Error> {
        let regular_grid = Self::read_grid_from_file(file_url, unstructured_grid)?;
        let layer = MeteoTempLayer::new(regular_grid);

        Ok(layer)
    }


    pub fn read_grid_from_file(file_url: &str, unstructured_grid: &UnstructuredGrid) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let regular_grid = FileToGridConverter::read_unstructured_grid_from_file_and_transform(
            file_url,
            Self::MISSING_VALUE,
            Self::transform_values,
            unstructured_grid,
        )?;

        Ok(regular_grid)
    }


    pub fn transform_values(value: f32) -> f32 {
        value
    }
}
