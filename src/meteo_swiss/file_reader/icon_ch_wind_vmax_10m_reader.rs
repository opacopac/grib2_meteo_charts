use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::grid::unstructured_value_grid::UnstructuredValueGrid;
use crate::meteo_swiss::meteo_swiss_error::MeteoSwissError;
use crate::system::file_helper::FileHelper;


pub struct IconChWindVmax10mReader;


impl IconChWindVmax10mReader {
    const MISSING_VALUE: f32 = -1.0;

    pub fn read_grid_from_file(file_url: &str, unstructured_grid: &UnstructuredGrid) -> Result<LatLonValueGrid<f32>, MeteoSwissError> {
        let mut reader = FileHelper::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let unstructured_values = doc.calculate_data_points(Self::MISSING_VALUE, Self::transform_values)?;
        let value_grid = UnstructuredValueGrid::new(unstructured_values, Self::MISSING_VALUE, unstructured_grid.clone());
        let regular_grid = value_grid.create_regular_grid();

        Ok(regular_grid)
    }


    pub fn transform_values(value: f32) -> f32 {
        value
    }
}
