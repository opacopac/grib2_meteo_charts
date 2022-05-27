use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_grid::LatLonGrid;
use crate::grib2::common::grib2_error::Grib2Error;
use crate::grib2::document::grib2_document::Grib2Document;
use crate::grib2::section3::grid_definition_template::GridDefinitionTemplate;

pub struct ValueGrid {
    pub data_points: Vec<f32>,
    pub grid: LatLonGrid
}


impl ValueGrid {
    pub const MISSING_VALUE: f32 = -1.0; // TODO

    pub fn from_grib2(document: Grib2Document) -> Result<ValueGrid, Grib2Error> {
        let data_points = document.calculate_data_points(Self::MISSING_VALUE)?;
        let grid= ValueGrid::get_grid(&document)?;

        let value_grid = ValueGrid {
            data_points,
            grid
        };

        return Ok(value_grid);
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> f32 {
        let idx = self.get_index_by_lat_lon(pos);
        let value = self.get_value_by_index(idx);

        return value;
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


    pub fn get_value_by_index(&self, index: usize) -> f32 {
        return if index >= self.data_points.len() {
            ValueGrid::MISSING_VALUE
        } else {
            self.data_points[index]
        }
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
            _ => Err(Grib2Error::InvalidData("invalid grid definition template, only LatLonGrid is supported.".to_string()))
        };
    }
}
