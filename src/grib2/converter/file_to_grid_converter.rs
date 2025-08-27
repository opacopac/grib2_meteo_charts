use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::converter::regular_grid_converter::RegularGridConverter;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use bzip2::read::BzDecoder;
use log::info;
use std::io::Read;

pub struct FileToGridConverter;


impl FileToGridConverter {
    pub fn read_grid_from_file(
        file_url: &str,
        missing_value: f32,
    ) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let mut reader = Self::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create(&doc, missing_value)?;

        Ok(grid)
    }


    pub fn read_grid_from_file_and_convert<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T,
    ) -> Result<LatLonValueGrid<T>, Grib2Error> {
        let mut reader = Self::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create_and_transform(&doc, missing_value, transform_fn)?;

        Ok(grid)
    }


    fn get_file_reader(filename: &str) -> impl Read {
        info!("reading file {}", filename);
        let response_result = ureq::get(filename)
            .call()
            .expect("Failed to get file from URL");
        let reader = response_result.into_body().into_reader();
        let bz_decoder = BzDecoder::new(reader);

        bz_decoder
    }
}
