use crate::geo::lat_lon::LatLon;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;

pub struct Grib2ToGridConverter;

impl Grib2ToGridConverter {
    pub fn get_lat_lon_values_from_grib_doc(
        lat_doc: &Grib2Document,
        lon_doc: &Grib2Document,
    ) -> Result<Vec<LatLon>, Grib2Error> {
        let lat_values = lat_doc.calculate_data_points(255.0, |x| x)?;
        let lon_values = lon_doc.calculate_data_points(255.0, |x| x)?;

        if lat_values.len() != lon_values.len() {
            return Err(Grib2Error::InvalidData(
                "number of lat and lon data points don't match".to_string(),
            ));
        }

        if lon_values.len() != lat_values.len() {
            return Err(Grib2Error::InvalidData(
                "number of clat/clon data points don't match".to_string(),
            ));
        }

        let coordinates: Vec<LatLon> = lat_values
            .iter()
            .zip(lon_values.iter())
            .map(|(&lat, &lon)| LatLon::new(lat, lon))
            .collect();

        Ok(coordinates)
    }
}
