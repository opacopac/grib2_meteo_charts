use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::regular_grid_converter::RegularGridConverter;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grid::unstructured_grid::UnstructuredGrid;
use crate::grid::unstructured_value_grid::UnstructuredValueGrid;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::ops::RangeInclusive;

pub struct FileToGridConverter;


impl FileToGridConverter {
    pub fn read_rectangular_grid_from_file(
        file_url: &str,
        missing_value: f32,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let doc = Grib2DocumentReader::read_single_doc_from_file(&file_url)?;
        let grid = RegularGridConverter::create(&doc, missing_value)?;

        Ok(grid)
    }


    pub fn read_rectangular_grid_from_file_and_transform<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T,
    ) -> Result<LatLonValueGrid<T>, Grib2Error> {
        let doc = Grib2DocumentReader::read_single_doc_from_file(&file_url)?;
        let grid = RegularGridConverter::create_and_transform(&doc, missing_value, transform_fn)?;

        Ok(grid)
    }


    pub fn read_unstructured_grid_from_file_and_transform<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T,
        unstructured_grid: &UnstructuredGrid,
    ) -> Result<LatLonValueGrid<T>, Grib2Error> {
        let doc = Grib2DocumentReader::read_single_doc_from_file(&file_url)?;
        let unstructured_values = doc.calculate_data_points(missing_value, transform_fn)?;
        let value_grid = UnstructuredValueGrid::new(unstructured_values, missing_value, unstructured_grid.clone());
        let regular_grid = value_grid.create_regular_grid();

        Ok(regular_grid)
    }


    pub fn read_multi_unstructured_grids_from_file_and_transform<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T,
        unstructured_grid: &UnstructuredGrid,
        filter_doc_indexes: Option<RangeInclusive<usize>>,
    ) -> Result<Vec<LatLonValueGrid<T>>, Grib2Error> {
        let docs = Grib2DocumentReader::read_multi_doc_from_file(&file_url)?;
        let filtered_docs: Vec<&_> = match filter_doc_indexes {
            Some(range) => docs
                .iter()
                .enumerate()
                .filter(|(i, _)| range.contains(i))
                .map(|(_, doc)| doc)
                .collect(),
            None => docs.iter().collect(),
        };

        let regular_grids = filtered_docs
            .into_par_iter()
            .map(|doc| {
                let unstructured_values = doc.calculate_data_points(missing_value, transform_fn)?;
                let value_grid = UnstructuredValueGrid::new(unstructured_values, missing_value, unstructured_grid.clone());
                let regular_grid = value_grid.create_regular_grid();

                Ok(regular_grid)
            })
            .collect::<Result<Vec<_>, Grib2Error>>()?;

        Ok(regular_grids)
    }
}
