use std::time::Instant;
use crate::geo::lat_lon::LatLon;
use crate::geo::unstructured_grid::UnstructuredGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::netcdf::data::netcdf_data_reader::NetCdfDataReader;
use crate::netcdf::document::netcdf_document_reader::NetCdfDocumentReader;

pub struct DwdIconGlobalGridReader;

const CLON_VAR_NAME: &str = "clon";
const CLAT_VAR_NAME: &str = "clat";


impl DwdIconGlobalGridReader {
    pub fn create(filename: &str) -> Result<UnstructuredGrid, Grib2Error> {
        let (doc, mut reader) = NetCdfDocumentReader::read_file(filename).unwrap(); // TODO
        let clat_data = NetCdfDataReader::read_data_by_var(&mut reader, &doc, CLAT_VAR_NAME).unwrap().get_doubles(); // TODO
        let clon_data = NetCdfDataReader::read_data_by_var(&mut reader, &doc, CLON_VAR_NAME).unwrap().get_doubles(); // TODO


        let start = Instant::now();
        let mut grid = UnstructuredGrid::new();
        for i in 0..clat_data.len() {
            let lat = clat_data[i].to_degrees() as f32;
            let lon = clon_data[i].to_degrees() as f32;
            let point = LatLon::new(lat, lon);
            grid.add_point_value(point, i);
        }
        println!("nodes: {}", grid.get_node_count());
        println!("populating grid: {}", start.elapsed().as_millis());

        return Ok(grid);
    }
}
