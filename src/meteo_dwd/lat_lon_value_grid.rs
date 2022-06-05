use std::ops::{Add, Mul};
use crate::geo::lat_lon::LatLon;
use crate::geo::lat_lon_extent::LatLonExtent;

pub struct LatLonValueGrid<T> {
    values: Vec<T>,
    missing_value: T,
    dimensions: (usize, usize),
    lat_lon_extent: LatLonExtent,
    lat_inc: f32,
    lon_inc: f32,
}


impl <T: Copy + PartialEq + Mul<f32, Output = T> + Add<Output = T>> LatLonValueGrid<T> {
    pub fn new(
        values: Vec<T>,
        missing_value: T,
        dimensions: (usize, usize),
        lat_lon_extent: LatLonExtent
    ) -> LatLonValueGrid<T> {
        let lat_inc = (lat_lon_extent.max_coord.lat - lat_lon_extent.min_coord.lat) / dimensions.1 as f32;
        let lon_inc = (lat_lon_extent.max_coord.lon - lat_lon_extent.min_coord.lon) / dimensions.0 as f32;

        return LatLonValueGrid { values, missing_value, dimensions, lat_lon_extent, lat_inc, lon_inc };
    }


    pub fn get_grid_dimensions(&self) -> (usize, usize) {
        return self.dimensions.clone();
    }


    pub fn get_grid_lat_lon_extent(&self) -> &LatLonExtent {
        return &self.lat_lon_extent;
    }


    pub fn get_value_by_xy(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.dimensions.0 || y >= self.dimensions.1 {
            return None
        }

        let idx = x + y * self.dimensions.0;
        let value = self.values[idx];

        return if value != self.missing_value {
            Some(value)
        } else {
            None
        }
    }


    pub fn get_value_by_lat_lon(&self, pos: &LatLon) -> Option<T> {
        if !self.lat_lon_extent.is_inside(pos) {
            return None;
        }

        let x = ((pos.lon - &self.lat_lon_extent.min_coord.lon) / &self.lon_inc).round() as usize;
        let y = ((pos.lat - &self.lat_lon_extent.min_coord.lat) / &self.lat_inc).round() as usize;
        let value = self.get_value_by_xy(x, y);

        return value;
    }


    pub fn interpolate_value_by_lat_lon(&self, pos: &LatLon) -> Option<T> {
        let x = (pos.lon - &self.lat_lon_extent.min_coord.lon) / &self.lon_inc;
        let y = (pos.lat - &self.lat_lon_extent.min_coord.lat) / &self.lat_inc;
        let x_floor_f32 = x.floor();
        let y_floor_f32 = y.floor();
        let x_ceil_f32 = x.ceil();
        let y_ceil_f32 = y.ceil();

        if x_ceil_f32 < 0.0 || y_ceil_f32 < 0.0 || x_floor_f32 >= self.dimensions.0 as f32 || y_floor_f32 >= self.dimensions.1 as f32 {
            return None;
        }

        let x_floor = x_floor_f32 as usize;
        let y_floor = y_floor_f32 as usize;
        let x_ceil = x_ceil_f32 as usize;
        let y_ceil = y_ceil_f32 as usize;

        if x_floor == x_ceil || y_floor == y_ceil {
            return self.get_value_by_xy(x_floor, y_floor);
        }

        let val_tl = self.get_value_by_xy(x_floor, y_floor);
        let val_tr = self.get_value_by_xy(x_ceil, y_floor);
        let val_bl = self.get_value_by_xy(x_floor, y_ceil);
        let val_br = self.get_value_by_xy(x_ceil, y_ceil);
        let val_t = Self::interpolate_value(val_tl, x_ceil_f32 - x, val_tr, x - x_floor_f32);
        let val_b = Self::interpolate_value(val_bl, x_ceil_f32 - x, val_br, x - x_floor_f32);
        let value = Self::interpolate_value(val_t, y_ceil_f32 - y, val_b, y - y_floor_f32);

        return value;
    }


    fn interpolate_value(value1: Option<T>, weight1: f32, value2: Option<T>, weight2: f32) -> Option<T> {
        if value1.is_none() || value2.is_none() {
            return None;
        }

        let val1 = value1.unwrap(); // TODO
        let val2 = value2.unwrap(); // TODO
        let value = val1 * weight1 + val2 * weight2;

        return Some(value);
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::lat_lon::LatLon;
    use crate::geo::lat_lon_extent::LatLonExtent;
    use crate::meteo_dwd::lat_lon_value_grid::LatLonValueGrid;


    fn create_test_grid() -> LatLonValueGrid<i32> {
        let values = vec![00, 01, 10, 11, -1, 21];
        let missing_value = -1;
        let dimensions = (2, 3);
        let lat_lon_extent = LatLonExtent::new(
            LatLon::new(40.0, 7.0),
            LatLon::new(46.0, 9.0)
        );

        return LatLonValueGrid::new(values, missing_value, dimensions, lat_lon_extent);
    }


    #[test]
    fn it_creates_a_new_instance_with_the_correct_lat_lon_incs() {
        let grid = create_test_grid();

        assert_eq!(2.0, grid.lat_inc);
        assert_eq!(1.0, grid.lon_inc);
    }


    #[test]
    fn it_gets_the_correct_dimensions() {
        let grid = create_test_grid();

        let result = grid.get_grid_dimensions();

        assert_eq!((2, 3), result);
    }


    #[test]
    fn it_gets_the_correct_lat_lon_extent() {
        let grid = create_test_grid();

        let result = grid.get_grid_lat_lon_extent();

        assert_eq!([40.0, 7.0], result.min_coord.as_array());
        assert_eq!([46.0, 9.0], result.max_coord.as_array());
    }


    #[test]
    fn it_gets_the_correct_x_y_value() {
        let grid = create_test_grid();

        let result00 = grid.get_value_by_xy(0, 0).unwrap();
        let result01 = grid.get_value_by_xy(1, 0).unwrap();
        let result21 = grid.get_value_by_xy(1, 2).unwrap();

        assert_eq!(0, result00);
        assert_eq!(1, result01);
        assert_eq!(21, result21);
    }


    #[test]
    fn it_gets_none_if_x_or_y_are_out_of_bounds() {
        let grid = create_test_grid();

        let result1 = grid.get_value_by_xy(2, 0);
        let result2 = grid.get_value_by_xy(0, 3);

        assert!(result1.is_none());
        assert!(result2.is_none());
    }


    #[test]
    fn it_gets_the_correct_lat_lon_value() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 7.0);
        let pos2 = LatLon::new(42.0, 7.0);
        let pos3 = LatLon::new(44.0, 8.0);
        let pos4 = LatLon::new(44.9, 8.4);

        let result1 = grid.get_value_by_lat_lon(&pos1).unwrap();
        let result2 = grid.get_value_by_lat_lon(&pos2).unwrap();
        let result3 = grid.get_value_by_lat_lon(&pos3).unwrap();
        let result4 = grid.get_value_by_lat_lon(&pos4).unwrap();

        assert_eq!(0, result1);
        assert_eq!(10, result2);
        assert_eq!(21, result3);
        assert_eq!(21, result4);
    }


    #[test]
    fn it_gets_none_if_lat_or_lon_are_out_of_bounds() {
        let grid = create_test_grid();
        let pos1 = LatLon::new(40.0, 6.9);
        let pos2 = LatLon::new(39.9, 7.0);
        let pos3 = LatLon::new(43.0, 9.1);
        let pos4 = LatLon::new(46.1, 8.0);
        let pos5 = LatLon::new(45.5, 8.5);

        let result1 = grid.get_value_by_lat_lon(&pos1);
        let result2 = grid.get_value_by_lat_lon(&pos2);
        let result3 = grid.get_value_by_lat_lon(&pos3);
        let result4 = grid.get_value_by_lat_lon(&pos4);
        let result5 = grid.get_value_by_lat_lon(&pos5);

        assert!(result1.is_none());
        assert!(result2.is_none());
        assert!(result3.is_none());
        assert!(result4.is_none());
        assert!(result5.is_none());
    }


    #[test]
    fn it_gets_none_for_missing_values() {
        let grid = create_test_grid();

        let result1 = grid.get_value_by_xy(0, 2);

        assert!(result1.is_none());
    }
}
