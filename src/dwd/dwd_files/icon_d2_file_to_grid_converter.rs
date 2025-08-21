use std::io::Read;

use bzip2::read::BzDecoder;
use log::info;

use crate::dwd::common::dwd_error::DwdError;
use crate::grib2::document::grib2_document_reader::Grib2DocumentReader;
use crate::grid::grid_value_type::GridValueType;
use crate::grid::lat_lon_value_grid::LatLonValueGrid;
use crate::grib2::converter::regular_grid_converter::RegularGridConverter;

pub struct IconD2FileToGridConverter;


impl IconD2FileToGridConverter {
    pub fn read_grid_from_file(file_url: &str) -> Result<LatLonValueGrid<f32>, DwdError> {
        let mut reader = Self::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create(&doc, -1.0)?;

        return Ok(grid);
    }


    pub fn read_grid_from_file_and_convert<T: GridValueType>(
        file_url: &str,
        missing_value: T,
        transform_fn: fn(f32) -> T
    ) -> Result<LatLonValueGrid<T>, DwdError> {
        let mut reader = Self::get_file_reader(&file_url);
        let doc = Grib2DocumentReader::read_single_doc_from_stream(&mut reader)?;
        let grid = RegularGridConverter::create_and_transform(&doc, missing_value, transform_fn)?;

        return Ok(grid);
    }


    fn get_file_reader(filename: &str) -> impl Read {
        info!("reading file {}", filename);
        let response_result = ureq::get(filename)
            .call()
            .expect("Failed to get file from URL");
        let reader = response_result.into_body().into_reader();
        let bz_decoder = BzDecoder::new(reader);

        return bz_decoder;
    }
}
