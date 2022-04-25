use crate::dwd::discipline_checker::DisciplineChecker;
use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_grid::LatLonGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section0::discipline::Discipline;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;
use crate::grib2::section4::meteo_parameter_category::MeteoParameterCategory;
use crate::grib2::section5::data_representation_template::DataRepresentationTemplate::GridPointDataSimplePacking;

pub struct DwdCloudCoverLayer {
    data_points: Vec<f32>,
    pub grid: LatLonGrid
}


impl DwdCloudCoverLayer {
    pub const MISSING_VALUE: f32 = -1.0; // TODO

    pub fn new(
        document: Grib2Document
    ) -> Result<DwdCloudCoverLayer, Grib2Error> {
        DisciplineChecker::check(
            &document,
            Discipline::Meteorological,
            MeteoParameterCategory::Cloud
        )?;

        let data_points = DwdCloudCoverLayer::calculate_data_points(&document)?;
        let grid= DwdCloudCoverLayer::get_grid(&document)?;

        let ccl = DwdCloudCoverLayer {
            data_points,
            grid
        };

        return Ok(ccl);
    }


    pub fn get_value_by_index(&self, index: usize) -> f32 {
        return if index >= self.data_points.len() {
            DwdCloudCoverLayer::MISSING_VALUE
        } else {
            self.data_points[index]
        }
    }


    pub fn get_index_by_lat_lon(&self, pos: &LatLon) -> usize {
        if !self.grid.is_pos_inside(pos) {
            return self.data_points.len();
        }

        let lat_idx = ((pos.lat - &self.grid.start_pos.lat) / &self.grid.lat_inc_deg).round() as u32;
        let lon_idx = ((pos.lon - &self.grid.start_pos.lon) / &self.grid.lon_inc_deg).round() as u32;

        /*println!("lat: {} lon: {}", pos.lat, pos.lon);
        println!("lat 1st: {} lon 1st: {}", self.first_grid_point.lat, self.first_grid_point.lon);
        println!("lat idx: {} lon idx: {}", lat_idx, lon_idx);*/

        let idx = (lat_idx * &self.grid.lon_grid_points + lon_idx) as usize;

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
                data_points.push(DwdCloudCoverLayer::MISSING_VALUE);
            }
        }

        return Ok(data_points);
    }


    fn get_grid(document: &Grib2Document) -> Result<LatLonGrid, Grib2Error> {
        return match &document.section3.grid_definition_template {
            GridDefinitionTemplate::LatitudeLongitude(tpl) => {
                Ok(LatLonGrid::new(
                    LatLon::new(tpl.first_grid_point_lat, tpl.first_grid_point_lon),
                    LatLon::new(tpl.last_grid_point_lat, tpl.last_grid_point_lon),
                    tpl.i_direction_increment,
                    tpl.j_direction_increment,
                    tpl.number_of_points_along_meridian,
                    tpl.number_of_points_along_parallel
                ))
            }
            _ => Err(Grib2Error::InvalidData(format!("invalid grid definition template")))
        };
    }
}
