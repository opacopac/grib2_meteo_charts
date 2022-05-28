use std::f32::consts::PI;

use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::meteo_dwd::lat_lon_value_grid::LatLonValueGrid;
use crate::netcdf::document::netcdf_document::NetCdfDocument;

pub static CLAT_VAR_NAME: &str = "clat";
pub static CLON_VAR_NAME: &str = "clon";


pub struct UnstructuredGridConverter;


impl UnstructuredGridConverter {
    const WIDTH_HEIGHT: usize = 4096;
    const POW: f32 = Self::WIDTH_HEIGHT as f32;

    pub fn create(grib2_doc: &Grib2Document, missing_value: f32, netcdf_doc: &NetCdfDocument) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        let unstructured_values = grib2_doc.calculate_data_points(missing_value)?;
        let (clon_values, clat_values) = Self::get_clat_clon_values(netcdf_doc)?;
        if clon_values.len() != unstructured_values.len() {
            return Err(Grib2Error::InvalidData("number of clat/clon and grib2 data points don't match".to_string()));
        }

        let lat_limit: f32 = PI.sinh().atan().to_degrees();
        let values = Self::calculate_structured_values(unstructured_values, missing_value, clon_values, clat_values, lat_limit);
        let extent = LatLonExtent {
            min_coord: LatLon { lat: -lat_limit, lon: -180.0 },
            max_coord: LatLon { lat: lat_limit, lon: 180.0 }
        };
        let grid = LatLonValueGrid::new(values, missing_value, (Self::WIDTH_HEIGHT, Self::WIDTH_HEIGHT), extent);

        return Ok(grid);
    }



    fn get_clat_clon_values(doc: &NetCdfDocument) -> Result<(Vec<f64>, Vec<f64>), Grib2Error> {
        if !doc.data_map.contains_key(CLAT_VAR_NAME) || !doc.data_map.contains_key(CLON_VAR_NAME) {
            return Err(Grib2Error::InvalidData("values clat / clon not found".to_string()));
        }

        let clat_values = doc.data_map.get(CLAT_VAR_NAME).unwrap().get_doubles();
        let clon_values = doc.data_map.get(CLON_VAR_NAME).unwrap().get_doubles();

        if clat_values.len() != clon_values.len() {
            return Err(Grib2Error::InvalidData("number of clat and clon data points don't match".to_string()));
        }

        return Ok((clat_values, clon_values));
    }


    fn calculate_structured_values(
        unstructured_values: Vec<f32>,
        missing_value: f32,
        clat_values: Vec<f64>,
        clon_values: Vec<f64>,
        lat_limit: f32
    ) -> Vec<f32> {
        let mut values = vec![missing_value; Self::WIDTH_HEIGHT * Self::WIDTH_HEIGHT];

        for i in 0..clat_values.len() {
            let lat = clat_values[i].to_degrees() as f32;
            if lat < -lat_limit || lat > lat_limit {
                continue;
            }

            let lon = clon_values[i].to_degrees() as f32;
            let idx = Self::calc_idx_from_latlon(lat, lon);
            values[idx] = unstructured_values[i];
        }

        return values;
    }


    fn calc_idx_from_latlon(lat: f32, lon: f32) -> usize {
        let x = ((lon + 180.0) / 360.0 * Self::POW).floor() as usize;
        let y = ((1.0 - (lat.to_radians().tan() + 1.0 / lat.to_radians().cos()).ln() / PI) / 2.0 * Self::POW).floor() as usize;
        let idx = x + y * Self::WIDTH_HEIGHT;

        return idx;
    }
}
