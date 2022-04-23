use crate::grib2::common::grib2_error::Grib2Error;
use crate::geo::lat_lon::LatLon;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate::GridPointDataSimplePacking;

pub struct CloudCoverLayer {
    data_points: Vec<f32>,
    first_grid_point: LatLon,
    i_direction_increment: f32,
    j_direction_increment: f32,
    number_of_points_along_parallel: u32,
    number_of_points_along_meridian: u32
}


impl CloudCoverLayer {
    pub const MISSING_VALUE: f32 = -1.0; // TODO

    pub fn new(
        document: Grib2Document
    ) -> Result<CloudCoverLayer, Grib2Error> {
        let data_points = CloudCoverLayer::calculate_data_points(&document)?;
        let (first_grid_point,
            i_direction_increment,
            j_direction_increment,
            number_of_points_along_parallel,
            number_of_points_along_meridian) = CloudCoverLayer::get_grid_definition(&document)?;

        let ccl = CloudCoverLayer {
            data_points,
            first_grid_point,
            i_direction_increment,
            j_direction_increment,
            number_of_points_along_parallel,
            number_of_points_along_meridian
        };

        return Ok(ccl);
    }


    pub fn lat_grid_points(&self) -> u32 {
        return self.number_of_points_along_meridian;
    }


    pub fn lon_grid_points(&self) -> u32 {
        return self.number_of_points_along_parallel;
    }


    pub fn lat_inc_deg(&self) -> f32 {
        return self.i_direction_increment;
    }


    pub fn lon_inc_deg(&self) -> f32 {
        return self.j_direction_increment;
    }


    pub fn first_grid_point(&self) -> LatLon {
        return self.first_grid_point.clone();
    }


    pub fn get_value_by_index(&self, index: usize) -> f32 {
        return if index > self.data_points.len() {
            CloudCoverLayer::MISSING_VALUE
        } else {
            self.data_points[index]
        }
    }


    pub fn get_index_by_lat_lon(&self, pos: &LatLon) -> usize {
        let lon_idx = (((pos.lon - &self.first_grid_point.lon + 360.0) % 360.0) / &self.j_direction_increment).round() as u32;
        let lat_idx = (((pos.lat - &self.first_grid_point.lat + 360.0) % 360.0) / &self.i_direction_increment).round() as u32;

        let idx = (lat_idx * &self.number_of_points_along_parallel + lon_idx) as usize;

        return idx;
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> f32 {
        let idx = self.get_index_by_lat_lon(pos);
        let value = self.get_value_by_index(idx);

        return value;
    }


    fn calculate_data_points(document: &Grib2Document) -> Result<Vec<f32>, Grib2Error> {
        let (ref_value, c1, c2) = match &document.section5.data_representation_template {
            GridPointDataSimplePacking(tpl) => {
                let c1 = (2 as f32).powi(tpl.binary_scale_factor_e as i32);
                let c2 = (10 as f32).powi(tpl.decimal_scale_factor_d as i32);
                (tpl.reference_value, c1, c2)
            }
            _ => return Err(Grib2Error::InvalidData(format!("invalid data representation template")))
        };

        let bitmap = &document.section6.bitmap;
        let raw_data_points = &document.section7.data_points;

        let mut data_points: Vec<f32> = vec![];
        let mut j = 0;
        for i in 0..document.section3.number_of_datapoints {
            let bitmask = 0b10000000 >> (i % 8);
            if bitmap[(i / 8) as usize] & bitmask > 0 {
                let raw_value = raw_data_points[j] as f32;
                let data_value = (ref_value + raw_value * c1) as f32 / c2;
                data_points.push(data_value);
                j += 1;
            } else {
                data_points.push(CloudCoverLayer::MISSING_VALUE);
            }
        }

        return Ok(data_points);
    }


    fn get_grid_definition(document: &Grib2Document) -> Result<(LatLon, f32, f32, u32, u32), Grib2Error> {
        return match &document.section3.grid_definition_template {
            GridDefinitionTemplate::LatitudeLongitude(tpl) => {
                Ok((
                    tpl.first_grid_point.clone(),
                    tpl.i_direction_increment,
                    tpl.j_direction_increment,
                    tpl.number_of_points_along_parallel,
                    tpl.number_of_points_along_meridian
                ))
            }
            _ => return Err(Grib2Error::InvalidData(format!("invalid grid definition template")))
        };
    }
}
