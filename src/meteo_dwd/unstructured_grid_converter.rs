use std::f32::consts::PI;

use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::meteo_dwd::lat_lon_value_grid::LatLonValueGrid;

pub struct UnstructuredGridConverter;


impl UnstructuredGridConverter {
    const WIDTH_HEIGHT: usize = 4096;
    const POW: f32 = Self::WIDTH_HEIGHT as f32;


    pub fn create(grib2_doc: &Grib2Document, missing_value: f32, clat_data: Vec<f64>, clon_data: Vec<f64>) -> Result<LatLonValueGrid<f32>, Grib2Error> {
        if clat_data.len() != clon_data.len() {
            return Err(Grib2Error::InvalidData("number of clat and clon data points don't match".to_string()));
        }

        let unstructured_values = grib2_doc.calculate_data_points(missing_value)?;
        if clat_data.len() != unstructured_values.len() {
            return Err(Grib2Error::InvalidData("number of clat/clon and grib2 data points don't match".to_string()));
        }

        let lat_limit: f32 = PI.sinh().atan().to_degrees();
        let values = Self::calculate_structured_values(unstructured_values, missing_value, clat_data, clon_data, lat_limit);
        let extent = LatLonExtent {
            min_coord: LatLon { lat: -lat_limit, lon: -180.0 },
            max_coord: LatLon { lat: lat_limit, lon: 180.0 }
        };
        let grid = LatLonValueGrid::new(values, (Self::WIDTH_HEIGHT, Self::WIDTH_HEIGHT), extent);

        return Ok(grid);
    }


    fn calculate_structured_values(
        unstructured_values: Vec<f32>,
        missing_value: f32,
        clat_data: Vec<f64>,
        clon_data: Vec<f64>,
        lat_limit: f32
    ) -> Vec<f32> {
        let mut values = vec![missing_value; Self::WIDTH_HEIGHT * Self::WIDTH_HEIGHT];

        for i in 0..clat_data.len() {
            let lat = clat_data[i].to_degrees() as f32;
            if lat < -lat_limit || lat > lat_limit {
                continue;
            }

            let lon = clon_data[i].to_degrees() as f32;
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
