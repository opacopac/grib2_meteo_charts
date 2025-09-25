use crate::geo::common::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::netcdf::document::netcdf_document::NetCdfDocument;

pub static CLAT_VAR_NAME: &str = "clat";
pub static CLON_VAR_NAME: &str = "clon";

pub struct NetCdftoGridConverter;

impl NetCdftoGridConverter {
    pub fn get_clat_clon_values_from_netcdf(
        doc: &NetCdfDocument,
    ) -> Result<(Vec<f64>, Vec<f64>), Grib2Error> {
        if !doc.data_map.contains_key(CLAT_VAR_NAME) || !doc.data_map.contains_key(CLON_VAR_NAME) {
            return Err(Grib2Error::InvalidData(
                "values clat / clon not found".to_string(),
            ));
        }

        let clat_values = doc.data_map.get(CLAT_VAR_NAME).unwrap().get_doubles();
        let clon_values = doc.data_map.get(CLON_VAR_NAME).unwrap().get_doubles();

        if clat_values.len() != clon_values.len() {
            return Err(Grib2Error::InvalidData(
                "number of clat and clon data points don't match".to_string(),
            ));
        }

        Ok((clat_values, clon_values))
    }

    pub fn get_lat_lon_values_from_netcdf2(
        doc: &NetCdfDocument,
    ) -> Result<Vec<LatLon>, Grib2Error> {
        if !doc.data_map.contains_key(CLAT_VAR_NAME) || !doc.data_map.contains_key(CLON_VAR_NAME) {
            return Err(Grib2Error::InvalidData(
                "values clat / clon not found".to_string(),
            ));
        }

        let clat_rad_values = doc.data_map.get(CLAT_VAR_NAME).unwrap().get_doubles();
        let clon_rad_values = doc.data_map.get(CLON_VAR_NAME).unwrap().get_doubles();

        if clat_rad_values.len() != clon_rad_values.len() {
            return Err(Grib2Error::InvalidData(
                "number of clat and clon data points don't match".to_string(),
            ));
        }

        let coordinates: Vec<LatLon> = clat_rad_values
            .iter()
            .zip(clon_rad_values.iter())
            .map(|(&lat, &lon)| LatLon::new(lat.to_degrees() as f32, lon.to_degrees() as f32))
            .collect();

        Ok(coordinates)
    }
}
