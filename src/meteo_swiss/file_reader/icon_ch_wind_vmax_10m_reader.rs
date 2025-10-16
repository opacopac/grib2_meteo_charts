use crate::geo::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::geo::grid::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::file_to_grid_converter::FileToGridConverter;


pub struct IconChWindVmax10mReader;


impl IconChWindVmax10mReader {
    const MISSING_VALUE: f32 = -1.0;

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
