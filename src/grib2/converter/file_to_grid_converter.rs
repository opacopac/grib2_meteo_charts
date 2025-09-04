use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::regular_grid_converter::RegularGridConverter;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::grid::unstructured_value_grid::UnstructuredValueGrid;
use crate::system::file_helper::FileHelper;


pub struct FileToGridConverter;


impl FileToGridConverter {
    pub fn read_rectangular_grid_from_file(
        file_url: &str,
        missing_value: f32,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let mut reader = FileHelper::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create(&doc, missing_value)?;

        Ok(grid)
    }


    pub fn read_rectangular_grid_from_file_and_convert<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T,
    ) -> Result<LatLonValueGrid<T>, Grib2Error> {
        let mut reader = FileHelper::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create_and_transform(&doc, missing_value, transform_fn)?;

        Ok(grid)
    }


    pub fn read_unstructured_grid_from_file_and_convert(
        file_url: &str,
        missing_value: f32,
        transform_fn: fn(f32) -> f32,
        unstructured_grid: &UnstructuredGrid
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let mut reader = FileHelper::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let unstructured_values = doc.calculate_data_points(missing_value, transform_fn)?;
        let value_grid = UnstructuredValueGrid::new(unstructured_values, missing_value, unstructured_grid.clone());
        let regular_grid = value_grid.create_regular_grid();

        Ok(regular_grid)
    }
}
